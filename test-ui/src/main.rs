use element_extractor::Extractor;
use jiff;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    cmp::max,
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder, EventLoopProxy},
    platform::macos::WindowAttributesExtMacOS,
    window::{Window, WindowId},
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    http::Response,
    Rect, WebView, WebViewBuilder,
};

pub const WEBVIEW_HEIGHT: u32 = 200;
pub const WEBVIEW_WIDTH: u32 = 50;
pub const CONTROL_PANEL_HEIGHT: u32 = 40;
pub const CONTROL_PANEL_WIDTH: u32 = 50;
pub const WINDOW_WIDTH: u32 = 240;
pub const RESIZE_DEBOUNCE_TIME: u128 = 500;

pub const TABBING_IDENTIFIER: &str = "New View"; // empty = no tabs, two separate windows are created

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoredView {
    url: String,
    title: String,
    index: usize,
    refresh_count: usize,
    last_refresh: jiff::Timestamp,
    refresh_interval: std::time::Duration,
    element_selector: Option<String>,
    element_values: Vec<ScrapeResult>,
    original_size: ViewSize,
    hidden: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ViewSize {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct JsScrapeResult {
    value: String,
    view_id: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ScrapeResult {
    element_value: String,
    timestamp: jiff::Timestamp,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct NanoId(String);

impl std::fmt::Display for NanoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct App {
    window: Option<Window>,
    new_view_form_window: Option<Window>,
    monitored_views: Arc<Mutex<Vec<MonitoredView>>>,
    webviews: Vec<wry::WebView>,
    element_views: HashMap<NanoId, ElementView>,
    controls: Vec<wry::WebView>,
    new_view_form: Option<wry::WebView>,
    proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    last_resize: Option<Instant>,
    // extractor: Extractor,
}

struct ElementView {
    webview: wry::WebView,
    nano_id: NanoId,
    visible: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ControlMessage {
    Refresh(usize),
    Add(AddWebView),
    Remove(usize),
    UpdateRefreshInterval(Seconds),
    Move(usize, Direction),
    ExtractResult(NanoId, String),
    Minimize(usize),
    ToggleElementView(NanoId),
    // Extract(String, String),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AddWebView {
    url: String,
    title: String,
    refresh_interval: Seconds,
}

type Seconds = i32;

impl App {
    fn calculate_window_height(&self) -> u32 {
        let view_count = self.monitored_views.lock().unwrap().len();
        if view_count == 0 {
            return WINDOW_WIDTH; // Default height for empty window
        }
        WINDOW_WIDTH // Let the window be resizable instead of calculating fixed height
    }

    fn refresh_webview(&mut self, index: usize) {
        if let Some(webview) = self.webviews.get_mut(index) {
            if let Err(e) = webview.reload() {
                warn!("Failed to reload webview {}: {:?}", index, e);
            }
        }
    }

    fn add_webview(&mut self, view: AddWebView) {
        if let Some(window) = self.window.as_ref() {
            // First update the window size for the new view
            let new_height = (self.webviews.len() + 1) as u32 * WEBVIEW_HEIGHT;
            window.request_inner_size(LogicalSize::new(WINDOW_WIDTH, new_height));

            // Get the updated size after the resize request
            let size = window.inner_size().to_logical::<u32>(window.scale_factor());

            let mut view = MonitoredView {
                url: if view.url.starts_with("https://") {
                    view.url
                } else {
                    format!("https://{}", view.url)
                },
                title: view.title,
                index: self.webviews.len(),
                refresh_count: 0,
                last_refresh: jiff::Timestamp::now(),
                refresh_interval: std::time::Duration::from_secs(view.refresh_interval as u64),
                element_selector: None,
                element_values: vec![],
                original_size: ViewSize {
                    width: size.width,
                    height: size.height,
                },
                hidden: false,
            };

            // Add to monitored views first so positions are calculated correctly
            let mut num_views = 0;
            {
                let mut views = self.monitored_views.lock().unwrap();
                views.push(view.clone());
                num_views = views.len();
            }
            // Create and add the webview and controls
            let webview = self.create_webview(&size, window, &mut view, num_views, num_views);

            if view.element_selector.is_some() {
                info!("Creating element view for {}", view.title);
                let starting_height = window.inner_size().height / num_views as u32;
                let starting_width = size.width;
                let element_view = WebViewBuilder::new()
                    .with_bounds(Rect {
                        position: LogicalPosition::new(0, starting_height * num_views as u32)
                            .into(),
                        size: LogicalSize::new(400, 400).into(),
                    })
                    .with_html(include_str!("../assets/element_view.html"))
                    .with_transparent(true)
                    .with_background_color((0, 0, 0, 0))
                    .build_as_child(window)
                    .unwrap();

                element_view.set_visible(true);
                self.element_views.insert(
                    NanoId(view.title.clone()),
                    ElementView {
                        webview: element_view,
                        nano_id: NanoId(view.title.clone()),
                        visible: false,
                    },
                );
            }

            let controls = {
                let webview_len = self.webviews.len();
                self.create_controls(
                    &size,
                    window,
                    webview_len,
                    self.proxy.clone(),
                    NanoId(view.title.clone()),
                )
            };

            self.webviews.push(webview);
            self.controls.push(controls);

            // Fix positions of all webviews to ensure proper layout
            self.fix_webview_positions();
        }
    }

    fn create_webview(
        &self,
        size: &LogicalSize<u32>,
        window: &Window,
        view: &mut MonitoredView,
        index: usize,
        num_views: usize,
    ) -> WebView {
        let proxy = self.proxy.clone();
        let starting_height = window.inner_size().height / num_views as u32;
        let starting_width = size.width;
        view.original_size = ViewSize {
            width: starting_width,
            height: starting_height,
        };

        let mut webviewbuilder = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, starting_height * index as u32).into(),
                size: LogicalSize::new(starting_width, starting_height).into(),
            })
            .with_visible(true)
            .with_clipboard(true)
            .with_ipc_handler(move |message| {
                match serde_json::from_str::<JsScrapeResult>(&message.body()) {
                    Ok(scrape_result) => {
                        info!("Scrape result: {:?}", scrape_result);
                        proxy
                            .lock()
                            .unwrap()
                            .send_event(UserEvent::ExtractResult(
                                NanoId(scrape_result.view_id.to_string()),
                                scrape_result.value,
                            ))
                            .unwrap();
                    }
                    Err(e) => {
                        warn!("Failed to parse message: {:?}", e);
                    }
                }
            })
            .with_visible(true);

        webviewbuilder = webviewbuilder.with_url(&view.url);
        let webview = webviewbuilder.build_as_child(window).unwrap();

        if let Some(selector) = &mut view.element_selector {
            info!("Extracting from {} with selector {}", view.url, selector);
            let script_content = include_str!("../assets/find_element.js")
                .replace("$pattern", &selector)
                .replace("$interval", &view.refresh_interval.as_millis().to_string())
                .replace("$view_id", &view.index.to_string());

            info!("Script content: {}", script_content);
            webview.evaluate_script(&script_content).unwrap();
            info!(
                "Extraction interval set for {} with selector {}",
                view.url, selector
            );
        }

        webview
    }

    fn remove_webview(&mut self, index: usize) {
        self.webviews.remove(index);
        self.controls.remove(index);
        self.monitored_views.lock().unwrap().remove(index);

        // Update window height
        if let Some(window) = self.window.as_ref() {
            let new_height = self.calculate_window_height();
            window.request_inner_size(LogicalSize::new(WINDOW_WIDTH, new_height));
        }

        self.fix_webview_positions();
    }

    fn fix_webview_positions(&mut self) {
        let window = self.window.as_ref().unwrap();
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());
        self.resize_webviews(&size);
    }

    fn create_controls(
        &self,
        size: &LogicalSize<u32>,
        window: &Window,
        i: usize,
        proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
        webview_id: NanoId,
    ) -> WebView {
        let script_contents = include_str!("../assets/controls.html")
            .replace("$0", &i.to_string())
            .replace("$id", &webview_id.to_string());
        debug!("Controls script contents: {}", script_contents);

        let control_panel = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, WEBVIEW_HEIGHT * i as u32).into(),
                size: LogicalSize::new(size.width, CONTROL_PANEL_HEIGHT).into(),
            })
            .with_html(script_contents)
            .with_ipc_handler(move |message| {
                info!("Received message: {:?}", message);

                let proxy = proxy.lock().unwrap();
                let message: ControlMessage = serde_json::from_str(&message.body()).unwrap();
                match message {
                    ControlMessage::Refresh(index) => {
                        proxy.send_event(UserEvent::Refresh(index)).unwrap();
                    }
                    ControlMessage::Add(view) => {
                        proxy.send_event(UserEvent::AddWebView(view)).unwrap();
                    }
                    ControlMessage::Remove(index) => {
                        proxy.send_event(UserEvent::RemoveWebView(index)).unwrap();
                    }
                    ControlMessage::UpdateRefreshInterval(_) => todo!(),
                    ControlMessage::Move(index, direction) => {
                        proxy
                            .send_event(UserEvent::MoveWebView(index, direction))
                            .unwrap();
                    }
                    ControlMessage::ExtractResult(view_id, result) => {
                        info!("Extracted result: {}", result);
                        proxy
                            .send_event(UserEvent::ExtractResult(view_id, result))
                            .unwrap();
                    }
                    ControlMessage::Minimize(index) => {
                        proxy.send_event(UserEvent::Minimize(index)).unwrap();
                    }
                    ControlMessage::ToggleElementView(nano_id) => {
                        info!("Toggling element view for {}", nano_id);
                        proxy
                            .send_event(UserEvent::ToggleElementView(nano_id))
                            .unwrap();
                    }
                }
            })
            .with_transparent(true)
            .with_background_color((0, 0, 0, 0))
            .build_as_child(window)
            .unwrap();

        control_panel
    }

    fn create_new_view_form(
        size: &LogicalSize<u32>,
        window: &Window,
        i: usize,
        proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    ) -> WebView {
        // put new view form at top of the window
        let control_panel = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: LogicalSize::new(size.width, size.height).into(),
            })
            .with_html(include_str!("../assets/new_view.html"))
            .with_ipc_handler(move |message| {
                info!("Received message: {:?}", message);

                let proxy = proxy.lock().unwrap();
                let message: ControlMessage = serde_json::from_str(&message.body()).unwrap();
                match message {
                    // ControlMessage::Refresh(index) => {
                    //     proxy.send_event(UserEvent::Refresh(index)).unwrap();
                    // }
                    ControlMessage::Add(view) => {
                        proxy.send_event(UserEvent::AddWebView(view)).unwrap();
                    }
                    _ => {}
                }
            })
            .with_transparent(true)
            .with_background_color((0, 0, 0, 0))
            .build_as_child(window)
            .unwrap();

        control_panel
    }

    fn resize_webviews(&mut self, size: &LogicalSize<u32>) {
        let window = self.window.as_ref().unwrap();
        let num_views = self.webviews.len();
        if num_views == 0 {
            return;
        }

        let webview_height = size.height / num_views as u32;

        // Pre-calculate common values
        let width = size.width;

        // Update all webviews in a single pass
        for i in 0..num_views {
            let y_position = webview_height * i as u32;

            // Only resize if the webview is visible
            if let Some(webview) = self.webviews.get_mut(i) {
                if let Ok(bounds) = webview.bounds() {
                    let current_size = bounds.size.to_logical::<u32>(window.scale_factor());
                    if current_size.width > 0 {
                        // Only resize visible webviews
                        webview.set_bounds(Rect {
                            position: LogicalPosition::new(0, y_position).into(),
                            size: LogicalSize::new(width, webview_height).into(),
                        });
                    }
                }
            }

            // Update control position
            if let Some(control) = self.controls.get_mut(i) {
                control.set_bounds(Rect {
                    position: LogicalPosition::new(0, y_position).into(),
                    size: LogicalSize::new(width, CONTROL_PANEL_HEIGHT).into(),
                });
            }
        }

        if let Some(new_view_form) = self.new_view_form.as_ref() {
            if let Some(new_form_window) = self.new_view_form_window.as_ref() {
                let new_form_size = new_form_window
                    .inner_size()
                    .to_logical::<u32>(window.scale_factor());
                new_view_form.set_bounds(Rect {
                    position: LogicalPosition::new(0, 0).into(),
                    size: LogicalSize::new(new_form_size.width, new_form_size.height).into(),
                });
            }
        }
    }

    fn move_webview(&mut self, index: usize, direction: Direction) {
        let new_index = match direction {
            Direction::Up => {
                if index + 1 >= self.webviews.len() {
                    return; // Can't move up if already at top
                }
                index + 1
            }
            Direction::Down => {
                if index == 0 {
                    return; // Can't move down if already at bottom
                }
                index - 1
            }
        };

        // Swap webviews
        self.webviews.swap(index, new_index);

        // Swap controls
        self.controls.swap(index, new_index);

        // Swap monitored views and update indices
        {
            let mut views = self.monitored_views.lock().unwrap();
            views.swap(index, new_index);
            views[index].index = index;
            views[new_index].index = new_index;
        } // Lock is dropped here

        // Update positions of all webviews and controls
        self.fix_webview_positions();
    }

    fn minimize_webview(&mut self, index: usize) {
        info!("Minimizing webview at index {}", index);
        let window = self.window.as_ref().unwrap();
        if let Some(webview) = self.webviews.get_mut(index) {
            if let Ok(bounds) = webview.bounds() {
                let original_size = self.monitored_views.lock().unwrap()[index]
                    .original_size
                    .clone();
                let current_size = bounds.size.to_logical::<u32>(window.scale_factor());
                info!(
                    "Current size, Original size: {:?}, {:?}",
                    current_size, original_size
                );
                if current_size.width > 0 {
                    webview.set_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: LogicalSize::new(0, 0).into(),
                    });
                } else {
                    webview.set_bounds(Rect {
                        position: LogicalPosition::new(0, 0).into(),
                        size: LogicalSize::new(
                            window.inner_size().width,
                            window.inner_size().height,
                        )
                        .into(),
                    });
                }
            }
        }
        {
            let mut views = self.monitored_views.lock().unwrap();
            views[index].hidden = true;
        }
    }

    fn update_element_view(&mut self, view_id: NanoId, result: String) {
        if let Some(element_view) = self.element_views.get_mut(&view_id) {
            // element_view.webview.set_html(result);
            element_view
                .webview
                .evaluate_script(&format!(
                    "document.getElementById('value').textContent = '{}';",
                    result
                ))
                .unwrap();
        }
    }
}

#[derive(Debug)]
enum UserEvent {
    Refresh(usize),
    AddWebView(AddWebView),
    RemoveWebView(usize),
    ShowNewViewForm,
    MoveWebView(usize, Direction),
    ExtractResult(NanoId, String),
    Minimize(usize),
    ToggleElementView(NanoId),
    // Extract(String, String),
    // ExtractResult(String),
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");
        let window_height = self.calculate_window_height();
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(WINDOW_WIDTH, window_height))
            .with_transparent(true)
            .with_blur(true)
            .with_movable_by_window_background(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(false)
            .with_titlebar_buttons_hidden(false)
            .with_titlebar_hidden(false)
            .with_title("Watcher")
            .with_has_shadow(false)
            .with_resizable(true);

        // controls window
        let new_view_form_window = event_loop
            .create_window(
                window_attributes
                    .clone()
                    .with_inner_size(LogicalSize::new(WINDOW_WIDTH, window_height))
                    .with_title("new view"),
            )
            .unwrap();

        let scale_factor = new_view_form_window.scale_factor();
        // let scale_factor = 0.5;
        let form_size = new_view_form_window
            .inner_size()
            .to_logical::<u32>(scale_factor);
        let new_view_form =
            App::create_new_view_form(&form_size, &new_view_form_window, 0, self.proxy.clone());
        self.new_view_form = Some(new_view_form);

        // creating the main window
        let window = event_loop
            .create_window(
                window_attributes.with_title("viewer"), // .with_title_hidden(true),
            )
            .expect("Failed to create window");
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        let mut num_views = 0;
        {
            num_views = self.monitored_views.lock().unwrap().len();
        }
        for (i, mut view) in self.monitored_views.lock().unwrap().iter_mut().enumerate() {
            info!("Creating webview for {}", view.url);
            let webview = self.create_webview(&size, &window, &mut view, i, num_views);
            let controls = self.create_controls(
                &size,
                &window,
                i,
                self.proxy.clone(),
                NanoId(view.title.clone()),
            );
            self.webviews.push(webview);
            self.controls.push(controls);
        }

        self.window = Some(window);
        self.new_view_form_window = Some(new_view_form_window);
        info!("Window and webviews created successfully");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // info!("Window event   received: {:?}", event);
        match event {
            WindowEvent::CloseRequested => {
                info!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // info!("Redraw requested");
                // window.request_redraw();
                // self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Debounce resize events to improve performance
                let now = Instant::now();
                if let Some(last_resize) = self.last_resize {
                    if now.duration_since(last_resize).as_millis() < RESIZE_DEBOUNCE_TIME {
                        // Skip this resize event if less than 50ms since last one
                        return;
                    }
                }
                self.last_resize = Some(now);

                let size = size.to_logical::<u32>(self.window.as_ref().unwrap().scale_factor());
                self.resize_webviews(&size);
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                info!("Keyboard input event: {:?}", event);
            }
            _ => {}
        }
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Refresh(index) => {
                info!("Refresh event received for index {}", index);
                self.refresh_webview(index);
            }
            UserEvent::AddWebView(view) => {
                info!("Adding new webview: {:?}", view);
                self.add_webview(view);
            }
            UserEvent::RemoveWebView(index) => {
                info!("Removing webview at index {}", index);
                self.remove_webview(index);
            }
            UserEvent::ShowNewViewForm => {
                info!("Showing new view form");
                self.new_view_form.as_ref().unwrap().set_visible(true);
            }
            UserEvent::MoveWebView(index, direction) => {
                info!("Moving webview at index {} {}", index, direction);
                self.move_webview(index, direction);
            }
            UserEvent::ExtractResult(view_id, result) => {
                info!("Extracted result: {}", result);
                self.update_element_view(view_id, result);
            }
            UserEvent::Minimize(index) => {
                info!("Minimizing webview at index {}", index);
                self.minimize_webview(index);
            }
            UserEvent::ToggleElementView(nano_id) => {
                info!("UserEvent: Toggling element view for {}", nano_id);
                if let Some(mut element_view) = self.element_views.get_mut(&nano_id) {
                    element_view.visible = !element_view.visible;
                    if element_view.visible {
                        element_view.webview.set_visible(true);
                    } else {
                        element_view.webview.set_visible(false);
                    }
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    info!("Starting application...");

    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    let event_loop_proxy = event_loop.create_proxy();
    let monitored_views = Arc::new(Mutex::new(vec![
        // MonitoredView {
        //     url: "https://google.com".to_string(),
        //     title: "Google".to_string(),
        //     index: 0,
        //     refresh_count: 0,
        //     last_refresh: jiff::Timestamp::now(),
        //     refresh_interval: std::time::Duration::from_secs(1),
        //     element_selector: None,
        // },
        MonitoredView {
            url: "https://finance.yahoo.com/quote/GME/".to_string(),
            title: "Price".to_string(),
            index: 1,
            refresh_count: 0,
            last_refresh: jiff::Timestamp::now(),
            refresh_interval: std::time::Duration::from_secs(5),
            element_selector: Some(r#"#nimbus-app > section > section > section > article > section.container.yf-5hy459 > div.bottom.yf-5hy459 > div.price.yf-5hy459 > section > div > section:nth-child(1) > div.container.yf-16vvaki > div:nth-child(1) > span"#.to_string()),
            element_values: vec![],
            original_size: ViewSize {
                width: 0,
                height: 0,
            },
            hidden: false,
        },
    ]));

    let monitored_views_clone = monitored_views.clone();
    let proxy_clone = event_loop_proxy.clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        for view in monitored_views_clone.lock().unwrap().iter_mut() {
            if view.last_refresh + view.refresh_interval < jiff::Timestamp::now() {
                view.refresh_count += 1;
                view.last_refresh = jiff::Timestamp::now();
                proxy_clone
                    .send_event(UserEvent::Refresh(view.index))
                    .unwrap();
            }
        }
    });

    // let extractor = Extractor::new();

    let mut app = App {
        window: None,
        new_view_form_window: None,
        monitored_views,
        webviews: vec![],
        controls: vec![],
        new_view_form: None,
        element_views: HashMap::new(),
        proxy: Arc::new(Mutex::new(event_loop_proxy)),
        last_resize: None,
        // extractor,
    };

    event_loop.run_app(&mut app).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_message() {
        let message = ControlMessage::Refresh(0);
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(json, r#"{"refresh":0}"#);

        // Test move command
        let message = ControlMessage::Move(1, Direction::Up);
        let json = serde_json::to_string(&message).unwrap();
        assert_eq!(json, r#"{"move":[1,"up"]}"#);

        // Test deserialization
        let message: ControlMessage = serde_json::from_str(r#"{"move":[1,"up"]}"#).unwrap();
        assert_eq!(message, ControlMessage::Move(1, Direction::Up));
    }

    #[test]
    fn test_add_webview() {
        let add_post_data = "{\"add\":{\"url\":\"\",\"refresh_interval\":\"\",\"title\":\"\"}}";

        let message: ControlMessage = serde_json::from_str(add_post_data).unwrap();

        assert_eq!(
            message,
            ControlMessage::Add(AddWebView {
                url: "".to_string(),
                refresh_interval: 60,
                title: "".to_string(),
            })
        );
    }
}

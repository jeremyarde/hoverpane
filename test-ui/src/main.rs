use jiff;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::{
    cmp::max,
    sync::{Arc, Mutex},
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
pub const CONTROL_PANEL_HEIGHT: u32 = 50;
pub const CONTROL_PANEL_WIDTH: u32 = 50;
pub const NEW_VIEW_FORM_HEIGHT: u32 = 150;
#[derive(Debug)]
enum CustomEvent {
    RefreshAll,
    RefreshTop,
    RefreshBottom,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoredView {
    url: String,
    title: String,
    index: usize,
    refresh_count: usize,
    last_refresh: jiff::Timestamp,
    refresh_interval: std::time::Duration,
}

struct App {
    window: Option<Window>,
    monitored_views: Arc<Mutex<Vec<MonitoredView>>>,
    webviews: Vec<wry::WebView>,
    controls: Vec<wry::WebView>,
    new_view_form: Option<wry::WebView>,
    proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ControlMessage {
    Refresh(usize),
    AddWebView(AddWebView),
    RemoveWebView(usize),
    UpdateRefreshInterval(Seconds),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AddWebView {
    url: String,
    title: String,
    refresh_interval: Seconds,
}

type Seconds = i32;

// enum ControlAction {
//     RefreshAll,
//     RefreshTop,
//     RefreshBottom,
// }

impl App {
    fn refresh_webview(&mut self, index: usize) {
        if let Some(webview) = self.webviews.get_mut(index) {
            if let Err(e) = webview.reload() {
                warn!("Failed to reload webview {}: {:?}", index, e);
            }
        }
    }

    fn add_webview(&mut self, view: AddWebView) {
        if let Some(window) = self.window.as_ref() {
            let size = window.inner_size().to_logical::<u32>(window.scale_factor());
            let view = MonitoredView {
                url: view.url,
                title: view.title,
                index: self.webviews.len(),
                refresh_count: 0,
                last_refresh: jiff::Timestamp::now(),
                refresh_interval: std::time::Duration::from_secs(view.refresh_interval as u64),
            };
            let webview = App::create_webview(&size, window, view.clone(), self.webviews.len() - 1);
            self.monitored_views.lock().unwrap().push(view);
            self.webviews.insert(self.webviews.len() - 1, webview);
        }
    }

    fn create_webview(
        size: &LogicalSize<u32>,
        window: &Window,
        view: MonitoredView,
        index: usize,
    ) -> WebView {
        let webview = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, WEBVIEW_HEIGHT * index as u32).into(),
                size: LogicalSize::new(size.width, WEBVIEW_HEIGHT).into(),
            })
            .with_url(&view.url)
            .build_as_child(window)
            .unwrap();
        // self.webviews.push(webview);
        webview
    }

    fn remove_webview(&mut self, index: usize) {
        self.webviews.remove(index);
        self.fix_webview_positions();
    }

    fn fix_webview_positions(&mut self) {
        let size = self
            .window
            .as_ref()
            .unwrap()
            .inner_size()
            .to_logical::<u32>(self.window.as_ref().unwrap().scale_factor());
        for (i, webview) in self.webviews.iter_mut().enumerate() {
            webview.set_bounds(Rect {
                position: LogicalPosition::new(0, WEBVIEW_HEIGHT * i as u32).into(),
                size: LogicalSize::new(size.width, WEBVIEW_HEIGHT).into(),
            });
        }
        for (i, control) in self.controls.iter_mut().enumerate() {
            control.set_bounds(Rect {
                position: LogicalPosition::new(0, CONTROL_PANEL_HEIGHT * i as u32).into(),
                size: LogicalSize::new(size.width, CONTROL_PANEL_HEIGHT).into(),
            });
        }
    }

    fn create_controls(
        size: &LogicalSize<u32>,
        window: &Window,
        i: usize,
        proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
    ) -> WebView {
        let control_panel = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, WEBVIEW_HEIGHT * i as u32).into(),
                size: LogicalSize::new(size.width / 2, CONTROL_PANEL_HEIGHT).into(),
            })
            .with_html(include_str!("../assets/controls.html").replace("$0", &i.to_string()))
            .with_ipc_handler(move |message| {
                info!("Received message: {:?}", message);

                let proxy = proxy.lock().unwrap();
                let message: ControlMessage = serde_json::from_str(&message.body()).unwrap();
                match message {
                    ControlMessage::Refresh(index) => {
                        proxy.send_event(UserEvent::Refresh(index)).unwrap();
                    }
                    ControlMessage::AddWebView(view) => {
                        proxy.send_event(UserEvent::AddWebView(view)).unwrap();
                    }
                    ControlMessage::RemoveWebView(index) => {
                        proxy.send_event(UserEvent::RemoveWebView(index)).unwrap();
                    }
                    ControlMessage::UpdateRefreshInterval(_) => todo!(),
                }
            })
            .with_transparent(true)
            .with_background_color((0, 0, 0, 0))
            // .with_initialization_script(
            //     "document.documentElement.style.background = 'transparent';",
            // )
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
                position: LogicalPosition::new(0, size.height - NEW_VIEW_FORM_HEIGHT).into(),
                size: LogicalSize::new(size.width, NEW_VIEW_FORM_HEIGHT).into(),
            })
            .with_html(include_str!("../assets/new_view.html"))
            .with_ipc_handler(move |message| {
                info!("Received message: {:?}", message);

                let proxy = proxy.lock().unwrap();
                let message: ControlMessage = serde_json::from_str(&message.body()).unwrap();
                match message {
                    ControlMessage::Refresh(index) => {
                        proxy.send_event(UserEvent::Refresh(index)).unwrap();
                    }
                    ControlMessage::AddWebView(view) => {
                        proxy.send_event(UserEvent::AddWebView(view)).unwrap();
                    }
                    ControlMessage::RemoveWebView(index) => {
                        proxy.send_event(UserEvent::RemoveWebView(index)).unwrap();
                    }
                    ControlMessage::UpdateRefreshInterval(_) => todo!(),
                }
            })
            .with_transparent(true)
            .with_background_color((0, 0, 0, 0))
            // .with_initialization_script(
            //     "document.documentElement.style.background = 'transparent';",
            // )
            .build_as_child(window)
            .unwrap();

        control_panel
    }

    fn resize_webviews(&mut self, size: &LogicalSize<u32>) {
        for (i, webview) in self.webviews.iter_mut().enumerate() {
            webview.set_bounds(Rect {
                position: LogicalPosition::new(
                    0,
                    (WEBVIEW_HEIGHT * i as u32).clamp(0, size.height),
                )
                .into(),
                size: LogicalSize::new(size.width, WEBVIEW_HEIGHT).into(),
            });
        }
        for (i, control) in self.controls.iter_mut().enumerate() {
            control.set_bounds(Rect {
                position: LogicalPosition::new(
                    0,
                    (CONTROL_PANEL_HEIGHT * i as u32).clamp(0, size.height),
                )
                .into(),
                size: LogicalSize::new(size.width, CONTROL_PANEL_HEIGHT).into(),
            });
        }
        if let Some(new_view_form) = self.new_view_form.as_ref() {
            new_view_form.set_bounds(Rect {
                position: LogicalPosition::new(
                    0,
                    (size.height - NEW_VIEW_FORM_HEIGHT).clamp(0, size.height),
                )
                .into(),
                size: LogicalSize::new(size.width, NEW_VIEW_FORM_HEIGHT).into(),
            });
        }
    }
}

#[derive(Debug)]
enum UserEvent {
    Refresh(usize),
    AddWebView(AddWebView),
    RemoveWebView(usize),
    ShowNewViewForm,
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(240.0, 720.0))
            .with_transparent(true)
            .with_blur(true)
            .with_movable_by_window_background(true)
            // .with_titlebar_transparent(true)
            .with_fullsize_content_view(true)
            .with_title_hidden(false)
            .with_titlebar_buttons_hidden(false)
            .with_titlebar_hidden(false)
            .with_has_shadow(true)
            .with_resizable(true);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        let new_view_form = App::create_new_view_form(&size, &window, 0, self.proxy.clone());
        self.new_view_form = Some(new_view_form);

        for (i, view) in self.monitored_views.lock().unwrap().iter_mut().enumerate() {
            info!("Creating webview for {}", view.url);
            let webview = App::create_webview(&size, &window, view.clone(), i);
            let controls = App::create_controls(&size, &window, i, self.proxy.clone());
            self.webviews.push(webview);
            self.controls.push(controls);
        }

        self.window = Some(window);
        info!("Window and webviews created successfully");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        info!("Window event received: {:?}", event);
        match event {
            WindowEvent::CloseRequested => {
                info!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                info!("Redraw requested");
                // window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                // info!("Window resized to {:?}", size);
                let size = size.to_logical::<u32>(self.window.as_ref().unwrap().scale_factor());
                self.resize_webviews(&size);
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
        }
    }
}

fn main() {
    env_logger::init();
    info!("Starting application...");

    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    let event_loop_proxy = event_loop.create_proxy();
    let monitored_views = Arc::new(Mutex::new(vec![
        MonitoredView {
            url: "https://google.com".to_string(),
            title: "Google".to_string(),
            index: 0,
            refresh_count: 0,
            last_refresh: jiff::Timestamp::now(),
            refresh_interval: std::time::Duration::from_secs(3),
        },
        MonitoredView {
            url: "https://hackernews.com".to_string(),
            title: "Hacker News".to_string(),
            index: 1,
            refresh_count: 0,
            last_refresh: jiff::Timestamp::now(),
            refresh_interval: std::time::Duration::from_secs(5),
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

    let mut app = App {
        window: None,
        monitored_views,
        webviews: vec![],
        controls: vec![],
        new_view_form: None,
        proxy: Arc::new(Mutex::new(event_loop_proxy)),
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
    }

    #[test]
    fn test_add_webview() {
        let add_post_data =
            "{\"addwebview\":{\"url\":\"\",\"refresh_interval\":\"\",\"title\":\"\"}}";

        let message: ControlMessage =
            serde_json::from_str(add_post_data.to_string().as_str()).unwrap();

        assert_eq!(
            message,
            ControlMessage::AddWebView(AddWebView {
                url: "".to_string(),
                refresh_interval: 60,
                title: "".to_string(),
            })
        );
    }
}

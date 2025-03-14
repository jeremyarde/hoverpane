use log::{info, warn};
use std::sync::{Arc, Mutex};
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
    Rect, WebViewBuilder,
};

// use std::cell::RefCell;
// thread_local! {
//     static APP: RefCell<Option<Arc<Mutex<App>>>> = RefCell::new(None);
// }

#[derive(Debug)]
enum CustomEvent {
    RefreshAll,
    RefreshTop,
    RefreshBottom,
}

#[derive(Debug)]
struct MonitoredView {
    url: String,
    title: String,
    index: usize,
    refresh_count: usize,
    last_refresh: std::time::Instant,
    refresh_interval: std::time::Duration,
}

struct App {
    window: Option<Window>,
    monitored_views: Arc<Mutex<Vec<MonitoredView>>>,
    webviews: Vec<wry::WebView>,
    proxy: Arc<Mutex<EventLoopProxy<UserEvent>>>,
}

impl App {
    fn refresh_webview(&mut self, index: usize) {
        // if let Some(webview) = self.webviews.get_mut(index) {
        //     if let Err(e) = webview.reload() {
        //         warn!("Failed to reload webview {}: {:?}", index, e);
        //     }
        // }
        // for webview in self.webviews.iter_mut() {
        //     if let Err(e) = webview.reload() {
        //         warn!("Failed to reload webview {}: {:?}", index, e);
        //     }
        // }
        if let Some(webview) = self.webviews.get_mut(index) {
            if let Err(e) = webview.reload() {
                warn!("Failed to reload webview {}: {:?}", index, e);
            }
        }
    }

    fn add_webview(&mut self, view: MonitoredView) {
        if let Some(window) = self.window.as_ref() {
            let size = window.inner_size().to_logical::<u32>(window.scale_factor());
            let index = self.webviews.len() - 1; // Subtract 1 to account for control panel

            info!("Creating new webview {} at index {}", view.url, index);
            let webview = WebViewBuilder::new()
                .with_bounds(Rect {
                    position: LogicalPosition::new(0, 150 * index as i32).into(),
                    size: LogicalSize::new(size.width, 150).into(),
                })
                .with_url(&view.url)
                .build_as_child(window)
                .unwrap();

            // Insert the new webview before the control panel
            self.webviews.insert(self.webviews.len() - 1, webview);
            self.monitored_views.lock().unwrap().push(view);
        }
    }

    fn remove_webview(&mut self, index: usize) {
        self.webviews.remove(index);
    }
}

#[derive(Debug)]
enum UserEvent {
    Refresh(usize),
    AddWebView(MonitoredView),
    RemoveWebView(usize),
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(240.0, 720.0))
            .with_transparent(true)
            .with_blur(true)
            .with_movable_by_window_background(true)
            .with_titlebar_transparent(true)
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

        for (i, view) in self.monitored_views.lock().unwrap().iter_mut().enumerate() {
            info!("Creating webview for {}", view.url);
            let webview = WebViewBuilder::new()
                .with_bounds(Rect {
                    position: LogicalPosition::new(0, 150 * i as i32).into(),
                    size: LogicalSize::new(size.width, (size.height - 50) / 2).into(),
                })
                .with_url(view.url.clone())
                .build_as_child(&window)
                .unwrap();
            self.webviews.push(webview);
        }

        let proxy_clone = self.proxy.clone();
        // Add window beside each of the webview, to delete the view, give it an outline/timer etc
        let control_panel = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: LogicalSize::new(size.width, 50).into(),
            })
            .with_html(
                r#"
                <style>
                    .controls {
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        padding: 0 16px;
                        height: 100%;
                        background: rgba(255, 255, 255, 0.9);
                    }
                    .button {
                        padding: 6px 12px;
                        border: none;
                        border-radius: 6px;
                        background: #007AFF;
                        color: white;
                        cursor: pointer;
                        font-size: 13px;
                    }
                    .button:hover {
                        opacity: 0.9;
                    }
                </style>
                <script>
                    function refresh(target) {
                        if (target === 'control') {
                            window.location.reload();
                        } else {
                            window.ipc.postMessage(target);
                        }
                    }
                </script>

                <div class="controls">
                    <button class="button" onclick="refresh('all')">Refresh All</button>
                    <button class="button" onclick="refresh('top')">Refresh Top</button>
                    <button class="button" onclick="refresh('bottom')">Refresh Bottom</button>
                </div>
            "#,
            )
            .with_ipc_handler(move |message| {
                info!("Received message: {:?}", message);
                proxy_clone
                    .lock()
                    .unwrap()
                    .send_event(UserEvent::AddWebView(MonitoredView {
                        url: "https://jeremyarde.com".to_string(),
                        title: "Jeremy Arde".to_string(),
                        index: 0,
                        refresh_count: 0,
                        last_refresh: std::time::Instant::now(),
                        refresh_interval: std::time::Duration::from_secs(3),
                    }))
                    .unwrap();
            })
            .build_as_child(&window)
            .unwrap();

        self.window = Some(window);
        self.webviews.push(control_panel);
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
            last_refresh: std::time::Instant::now(),
            refresh_interval: std::time::Duration::from_secs(3),
        },
        MonitoredView {
            url: "https://hackernews.com".to_string(),
            title: "Hacker News".to_string(),
            index: 1,
            refresh_count: 0,
            last_refresh: std::time::Instant::now(),
            refresh_interval: std::time::Duration::from_secs(5),
        },
    ]));

    let proxy_clone = event_loop_proxy.clone();
    std::thread::spawn(move || {
        let mut counter = 0;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(15));

            let new_view = MonitoredView {
                url: format!("https://example.com/{}", counter),
                title: format!("Example {}", counter),
                index: counter + 2, // +2 because we start with 2 views
                refresh_count: 0,
                last_refresh: std::time::Instant::now(),
                refresh_interval: std::time::Duration::from_secs(3),
            };

            info!("Adding new webview {}", new_view.title);
            proxy_clone
                .send_event(UserEvent::AddWebView(new_view))
                .unwrap();
            counter += 1;
        }
    });

    let mut app = App {
        window: None,
        monitored_views,
        webviews: vec![],
        proxy: Arc::new(Mutex::new(event_loop_proxy)),
    };

    event_loop.run_app(&mut app).unwrap();
}

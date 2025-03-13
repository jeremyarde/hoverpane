use log::{info, warn};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    platform::macos::WindowAttributesExtMacOS,
    window::{Window, WindowId},
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    http::Response,
    Rect, WebViewBuilder,
};

#[derive(Default)]
struct App {
    window: Option<Window>,
    webviews: Vec<wry::WebView>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");
        let window_attributes = Window::default_attributes()
            .with_inner_size(LogicalSize::new(240.0, 720.0))
            .with_transparent(true)
            .with_blur(true)
            // .with_title("Element Viewer")
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

        // Adjust the positions of the content webviews to account for the control panel
        let webview1 = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 50).into(),
                size: LogicalSize::new(size.width, (size.height - 50) / 2).into(),
            })
            .with_url("https://google.com")
            .build_as_child(&window)
            .unwrap();

        let webview2 = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 50 + (size.height - 50) / 2).into(),
                size: LogicalSize::new(size.width, (size.height - 50) / 2).into(),
            })
            .with_url("https://hackernews.com")
            .with_on_page_load_handler(handler)
            // inserts a button that reloads the page
            .with_initialization_script(
                r#"
                function refreshAll() {
                    window.location.reload();
                }

                document.body.innerHTML = '<button onclick="refreshAll()">Refresh</button>';
            "#,
            )
            .build_as_child(&window)
            .unwrap();

        // let control_panel = WebViewBuilder::new()
        //     .with_bounds(Rect {
        //         position: LogicalPosition::new(0, 0).into(),
        //         size: LogicalSize::new(size.width, 50).into(),
        //     })
        //     .with_html(r#"
        //         <div style="display: flex; justify-content: space-between; align-items: center; padding: 0 16px;">
        //             <button class="button" onclick="refreshAll()">Refresh All</button>
        //             <span class="timer-display" id="timer"></span>
        //         </div>
        //     "#)
        //     .build_as_child(&window)
        //     .unwrap();

        self.window = Some(window);
        // self.webviews.push(control_panel);
        self.webviews.push(webview1);
        self.webviews.push(webview2);
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
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();
    info!("Starting application...");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

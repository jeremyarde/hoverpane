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
    Rect, WebViewBuilder,
};

#[derive(Default)]
struct App {
    window: Option<Window>,
    webviews: Vec<wry::WebView>,
    // webview: Option<wry::WebView>,
    // webview2: Option<wry::WebView>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");
        let window_attributes = Window::default_attributes()
            // .with_title("Message Viewer")
            // .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            // .with_visible(true)
            .with_inner_size(LogicalSize::new(240.0, 720.0))
            .with_transparent(true)
            .with_blur(true)
            .with_title("Element Viewer")
            .with_movable_by_window_background(true)
            .with_resizable(true);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        let webview1 = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: LogicalSize::new(size.width, size.height / 2).into(),
            })
            .with_url("https://google.com")
            .build_as_child(&window)
            .unwrap();

        let webview2 = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, size.height / 2).into(),
                size: LogicalSize::new(size.width, size.height / 2).into(),
            })
            .with_url("https://hackernews.com")
            .build_as_child(&window)
            .unwrap();

        // let webview = WebViewBuilder::new()
        //     .with_html(
        //         r#"
        //         <div>
        //             <h1>Hello, world!</h1>
        //         </div>
        //     "#,
        //     )
        //     .build(&window)
        //     .expect("Failed to create webview");

        self.window = Some(window);
        self.webviews.push(webview1);
        self.webviews.push(webview2);
        // self.webview = Some(webview);
        info!("Window and webview created successfully");
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

    // fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
    //     info!("Application suspended");
    //     // Clean up resources when suspended
    //     if self.window.is_some() {
    //         self.window.take();
    //     }
    //     if self.webview.is_some() {
    //         self.webview.take();
    //     }
    //     // if let Some(window) = self.window.take() {
    //     //     // window.close();
    //     // }
    //     // if let Some(webview) = self.webview.take() {
    //     //     // webview.close();
    //     // }
    // }

    // fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
    //     info!("Application exiting");
    //     // Clean up resources when exiting
    //     if self.webview.is_some() {
    //         self.webview.take();
    //     }
    //     if self.window.is_some() {
    //         self.window.take();
    //     }
    // }
}

fn main() {
    env_logger::init();
    info!("Starting application...");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

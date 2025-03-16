use std::io::Read as _;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use dioxus::desktop::{window, Config, WindowBuilder};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

pub fn app() -> Element {
    // let ctx = use_context::<Arc<TcpListener>>();
    let mut message = use_signal(|| "".to_string());
    let mut running = use_signal(|| false);

    info!("Starting App");
    // let mut reader = use_coroutine(move || async move {
    //     info!("Starting listener");
    //     // let mut reader = std::net::TcpListener::bind("127.0.0.1:9090").unwrap();
    //     //     let mut reader = std::net::TcpStream::connect("127.0.0.1:9090").unwrap();
    //     //     info!("Server listening on 127.0.0.1:9090");

    //     loop {
    //         let mut buf = [0; 1024];
    //         let bytes_read = reader.read(&mut buf).unwrap();
    //         let new_message = String::from_utf8_lossy(&buf[..bytes_read]);
    //         std::thread::sleep(std::time::Duration::from_secs(5));
    //         message.set(new_message.to_string());
    //     }
    // });

    // use_future(move || async move {
    //     loop {
    //         if running() {
    //             let mut stream = std::net::TcpStream::connect("127.0.0.1:9090").unwrap();
    //             let mut reader = std::io::BufReader::new(stream);
    //             let mut buf = [0; 1024];
    //             let bytes_read = reader.read(&mut buf).unwrap();
    //             let new_message = String::from_utf8_lossy(&buf[..bytes_read]);
    //             message.set(new_message.to_string());
    //         }
    //         std::thread::sleep(std::time::Duration::from_secs(5));
    //     }
    // });

    // let client = use_coroutine(|mut rx: UnboundedReceiver<String>| async move {
    //     while let Some(message) = rx.next().await {
    //         info!("Message: {}", message);
    //     }
    // });

    let mut count = use_signal(|| 0);

    rsx! {
        // document::Link { href: asset!("/assets/hello.css"), rel: "stylesheet" }
        iframe { src: "https://www.google.com" }
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        button { onclick: move |_| async move {}, "Run a server function!" }
        button { onclick: move |_| running.set(!running()), "Run a server function!" }
        "Server said: {message}"
    }
}

fn main() {
    // let val = String::from("Hello, world!");
    // dioxus::launch(app);
    env_logger::init();

    dioxus::LaunchBuilder::desktop()
        // .with_cfg(Config::new().with_window(|w| w.with_title("My App")))
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("My App")
                    .with_always_on_top(true),
            ),
        )
        // .with_context(Arc::new(val))
        // .with_context(Arc::clone(&ui_state))
        .launch(app);
}

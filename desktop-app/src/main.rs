use futures_util::{SinkExt, StreamExt};
use log::{error, info, LevelFilter};
use scraper::{Html, Selector};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Clone, Debug)]
struct SelectedElementDetails {
    display_text: String,
    selector: String,
    text: String,
    url: String,
    timestamp: String,
}

// Define our application state
#[derive(Default)]
struct AppState {
    tracked_elements: Vec<SelectedElement>,
    is_running: bool,
    tracked_element_details: HashMap<String, SelectedElementDetails>,
}

// Text("{\"selector\":\"#ember4\",\"text\":\"crates.io\",\"url\":\"https://crates.io/crates/headless_chrome\",\"timestamp\":\"2025-03-11T21:57:40.013Z\",\"id\":\"1741731501763-ml89r48rk\"}")

#[derive(Clone, Debug)]
struct SelectedElement {
    selector: String,
    text: String,
    url: String,
    timestamp: String,
    id: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            tracked_elements: Vec::new(),
            is_running: true,
            tracked_element_details: HashMap::new(),
        }
    }

    fn add_element(&mut self, element: Value) {
        let element = SelectedElement {
            selector: element["selector"].as_str().unwrap_or_default().to_string(),
            text: element["text"].as_str().unwrap_or_default().to_string(),
            url: element["url"].as_str().unwrap_or_default().to_string(),
            timestamp: element["timestamp"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            id: element["id"].as_str().unwrap_or_default().to_string(),
        };
        self.tracked_elements.push(element);
    }

    fn get_elements(&self) -> Vec<SelectedElement> {
        self.tracked_elements.clone()
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::builder().filter_level(LevelFilter::Info).init();

    info!("Starting WebSocket server...");

    // Create shared state
    let state = Arc::new(Mutex::new(AppState::new()));

    // Clone state for the frontend thread
    let frontend_state = Arc::clone(&state);
    let ui_state = Arc::clone(&state);

    // Spawn the frontend thread
    std::thread::spawn(move || {
        run_frontend(frontend_state);
    });

    // std::thread::spawn(move || {
    //     send_to_ui(ui_state);
    // });

    // Define the address to listen on
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();

    // Create the TCP listener
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    info!("WebSocket server listening on: {}", addr);

    // Accept incoming connections
    while let Ok((stream, peer)) = listener.accept().await {
        info!("Incoming connection from: {}", peer);
        let connection_state = Arc::clone(&state);
        tokio::spawn(handle_connection(stream, peer, connection_state));
    }
}

// async fn send_to_ui(state: Arc<Mutex<AppState>>) {
//     let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
//     // let (mut write, mut read) = stream.split();

//     let mut buf = [0; 1024];

//     while let Ok(bytes_read) = stream.read(&mut buf) {
//         if bytes_read == 0 {
//             break;
//         }
//         let message = String::from_utf8_lossy(&buf[..bytes_read]);
//         info!("Received message from: {:?}", message);
//     }
// }

async fn handle_connection(stream: TcpStream, peer: SocketAddr, state: Arc<Mutex<AppState>>) {
    match accept_async(stream).await {
        Ok(ws_stream) => {
            info!("WebSocket connection established with: {}", peer);
            let (mut write, mut read) = ws_stream.split();

            // Handle incoming messages
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        if msg.is_text() {
                            info!("Received message from {}: {:?}", peer, msg);

                            // Try to parse the message as JSON
                            if let Ok(mut json_value) =
                                serde_json::from_str(msg.to_text().unwrap_or_default())
                            {
                                println!("Parsed message: {:?}", json_value);
                                // Extract message ID if present
                                let message_id = if let Value::Object(ref mut obj) = json_value {
                                    obj.remove("id")
                                } else {
                                    None
                                };

                                // Lock the state and add the new element
                                if let Ok(mut app_state) = state.lock() {
                                    app_state.add_element(json_value.clone());
                                    info!(
                                        "Added new element to state. Total elements: {}",
                                        app_state.tracked_elements.len()
                                    );
                                }

                                // Send acknowledgment if we have a message ID
                                if let Some(id) = message_id {
                                    let ack = json!({
                                        "ack": id,
                                        "status": "success"
                                    });
                                    if let Err(e) = write.send(Message::Text(ack.to_string())).await
                                    {
                                        error!("Error sending ack to {}: {}", peer, e);
                                    }
                                }
                            } else {
                                error!("Could not parse message as JSON: {:?}", msg);
                                // Send error response
                                let error_response = json!({
                                    "error": "Invalid JSON format",
                                    "status": "error"
                                });
                                if let Err(e) =
                                    write.send(Message::Text(error_response.to_string())).await
                                {
                                    error!("Error sending error response to {}: {}", peer, e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error receiving message from {}: {}", peer, e);
                        break;
                    }
                }
            }
            info!("Connection closed with: {}", peer);
        }
        Err(e) => {
            error!("Error during WebSocket handshake with {}: {}", peer, e);
        }
    }
}

fn run_frontend(state: Arc<Mutex<AppState>>) {
    // let mut listener = std::net::TcpStream::connect("127.0.0.1:9090").unwrap();
    let listener_address = "127.0.0.1:9090";
    info!("Starting listener on {}", listener_address);
    let listener = std::net::TcpListener::bind(listener_address).unwrap();
    let (mut stream, _test) = listener.accept().unwrap();

    let res = stream.write(b"Hello, world this is a test!");
    println!("stream res: {:?}", res);

    // Example of how to access state from the frontend thread
    loop {
        if let Ok(mut app_state) = state.lock() {
            info!("App state: {:?}", app_state.get_elements().len());
            if !app_state.is_running {
                break;
            }

            for element in app_state.get_elements() {
                info!("Element: {:?}", element);
                let client_response = reqwest::blocking::Client::builder()
                    .user_agent("gethashdown.com")
                    .build()
                    .unwrap()
                    .get(element.url)
                    .send();

                // if response.is_err() {
                //     info!("Error: {:?}", response.err());
                //     continue;
                // }
                let mut response = match client_response {
                    Ok(response) => response,
                    Err(e) => {
                        info!("Error: {:?}", e);
                        continue;
                    }
                };

                info!("Response status: {:?}", &response.status());
                let text = response.text().unwrap();
                info!("Response text: {:?}", &text);

                let document = Html::parse_document(&text);
                let selector = Selector::parse(&element.selector).unwrap();

                let mut elements = document.select(&selector);

                for element in elements {
                    println!("Selected details: {:?}", element.text().collect::<Vec<_>>());
                    let details = SelectedElementDetails {
                        display_text: element.text().collect::<Vec<_>>().join(" "),
                        selector: element.attr("id").unwrap_or_default().to_string(),
                        text: element.text().collect::<Vec<_>>().join(" "),
                        url: element.attr("href").unwrap_or_default().to_string(),
                        timestamp: element.attr("timestamp").unwrap_or_default().to_string(),
                    };
                    app_state
                        .tracked_element_details
                        .insert(element.attr("id").unwrap_or_default().to_string(), details);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

    info!("Frontend thread stopped");
}

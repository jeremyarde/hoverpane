use log::info;
use widget_types::{ApiAction, EventSender, EventSenderImpl};
use winit::event_loop::EventLoopProxy;

#[derive(Clone)]
pub struct WinitEventSender {
    proxy: EventLoopProxy<super::UserEvent>,
}

impl WinitEventSender {
    pub fn new(proxy: EventLoopProxy<super::UserEvent>) -> Self {
        Self { proxy }
    }

    pub fn into_event_sender(self) -> EventSender {
        EventSender {
            inner: Box::new(self),
        }
    }
}

impl EventSenderImpl for WinitEventSender {
    fn send_message(&self, message: ApiAction) -> Result<(), String> {
        info!("Sending message");
        self.proxy
            .send_event(super::UserEvent::ApiAction(message))
            .map_err(|e| e.to_string())
    }
}

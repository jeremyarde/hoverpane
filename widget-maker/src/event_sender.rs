use widget_types::{EventSender, EventSenderImpl, ScrapedValue};
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
    fn send_scrape_result(&self, value: ScrapedValue) -> Result<(), String> {
        self.proxy
            .send_event(super::UserEvent::ExtractResult(value))
            .map_err(|e| e.to_string())
    }
}

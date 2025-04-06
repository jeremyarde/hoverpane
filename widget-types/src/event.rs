use crate::ScrapedValue;

pub struct EventSender {
    // This will be a wrapper around the actual sender implementation
    // The implementation details will be in widget-maker
    #[doc(hidden)]
    pub inner: Box<dyn EventSenderImpl + Send + Sync>,
}

impl EventSender {
    pub fn send_scrape_result(&self, value: ScrapedValue) -> Result<(), String> {
        self.inner.send_scrape_result(value)
    }
}

// This trait will be implemented by widget-maker
#[doc(hidden)]
pub trait EventSenderImpl: EventSenderImplClone {
    fn send_scrape_result(&self, value: ScrapedValue) -> Result<(), String>;
}

pub trait EventSenderImplClone {
    fn clone_box(&self) -> Box<dyn EventSenderImpl + Send + Sync>;
}

impl<T> EventSenderImplClone for T
where
    T: 'static + EventSenderImpl + Clone + Send + Sync,
{
    fn clone_box(&self) -> Box<dyn EventSenderImpl + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for EventSender {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_box(),
        }
    }
}

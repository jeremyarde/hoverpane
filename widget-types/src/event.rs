use crate::WidgetConfiguration;

pub struct EventSender {
    // This will be a wrapper around the actual sender implementation
    // The implementation details will be in hoverpane
    #[doc(hidden)]
    pub inner: Box<dyn EventSenderImpl + Send + Sync>,
}

impl EventSender {
    pub fn send_message(&self, message: ApiAction) -> Result<(), String> {
        self.inner.send_message(message)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiAction {
    DeleteWidget(String),
    CreateWidget(WidgetConfiguration),
    // DeleteWidgetModifier(String, String),
}

// This trait will be implemented by hoverpane
#[doc(hidden)]
pub trait EventSenderImpl: EventSenderImplClone {
    fn send_message(&self, message: ApiAction) -> Result<(), String>;
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

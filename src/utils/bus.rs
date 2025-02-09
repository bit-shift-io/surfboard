use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;


type EventHandler = Box<dyn Fn() -> String + Send + Sync>;

pub static BUS: Lazy<Arc<Mutex<EventBus>>> = Lazy::new(|| Arc::new(Mutex::new(EventBus::new())));


// the target class to execute the dispatch
#[derive(PartialEq)]
pub enum Target {
    ObjectA,
    ObjectB,
    // Add other targets as needed
}

pub struct EventBus {
    listeners: Vec<(Target, EventHandler)>, // Storing closures mapped to events
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("listeners", &self.listeners.len())
            .finish()
    }
}


impl EventBus {
    pub fn new() -> Self {
        EventBus {
            listeners: Vec::new(),
        }
    }

    // Subscribe a class to the bus
    pub fn subscribe<F>(&mut self, target: Target, handler: F)
    where
        F: Fn() -> String + 'static + Send + Sync,
    {
        self.listeners.push((target, Box::new(handler)));
    }

    pub fn dispatch(&self, target: Target, data: i32) -> String {  // Changed to accept data
        let mut results = Vec::new(); // Store results from handlers

        for (listener_target, handler) in &self.listeners {
            if *listener_target == target {
                results.push(handler()); // Call the handler and collect the result
            }
        }

        // Combine or process the results as needed.  For now, just return the first one, or an empty string if there are no handlers.
        results.into_iter().next().unwrap_or_default()
    }
}
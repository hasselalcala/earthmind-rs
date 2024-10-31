#[derive(Debug)]
pub enum Log {
    Event {
        event_name: String,
        data: Vec<(&'static str, serde_json::Value)>,
    },
    Message(String),
}

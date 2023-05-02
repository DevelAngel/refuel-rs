use chrono::{DateTime, Utc};

pub(crate) struct RefuelStation {
    pub name: String,
    pub addr: String,
    pub price: f32,
    pub updated: DateTime<Utc>,
}

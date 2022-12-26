use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct PageAdapter<T> {
    pub items: Vec<T>,
    pub page: u64,
    pub page_size: u64
}
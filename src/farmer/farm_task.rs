use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmTask {
    pub id: String,
    pub url: String,
    pub price: usize
}

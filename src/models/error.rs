use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseError {
    pub error: String,
}

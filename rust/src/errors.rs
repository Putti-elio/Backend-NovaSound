use serde::{Serialize};

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    details: Option<String>, 
}
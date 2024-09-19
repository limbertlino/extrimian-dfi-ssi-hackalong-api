use rocket::{
    http::Status,
    serde::json::{json, Json, Value},
};

use crate::models::Ticket;
use crate::services::get_issuance_invitation_code;

/// Simple endpoint to check server Health
#[get("/health")]
pub fn check_health() -> &'static str {
    "Ok"
}

#[put("/issue-vc", format = "json", data = "<ticket>")]
/// Endpoint to issue a new credential based on ticket information
pub async fn create_new_vc(ticket: Json<Ticket>) -> Result<Value, Status> {
    match get_issuance_invitation_code(ticket.0).await {
        Ok(json) => Ok(json!(json)),
        Err(_) => Err(Status::InternalServerError),
    }
}

use rocket::serde::json::{json, Value};

use crate::models::Ticket;

use log::error;
use reqwest::Error;
use std::fs;

use crate::utils::{get_category_data, update_common_fields};

/// Fetches an issuance invitation code by making a request with the ticket details
pub async fn get_issuance_invitation_code(ticket: Ticket) -> Result<Value, Error> {
    let client = reqwest::Client::new();

    let base_url = "https://sandbox-ssi.extrimian.com/v1/credentialsbbs/wacioob";

    // Read the JSON template from a file.
    let string_local_json_data =
        fs::read_to_string("json_template/ticket_template.json").expect("Can't read the json file");

    // Parse the JSON template string into a JSON value.
    let mut json_value: Value =
        serde_json::from_str(&string_local_json_data).expect("JSON was not well-formatted");

    // Get the category data associated with the ticket's category.
    let category = &ticket.category;
    let data = get_category_data(category);

    // Update JSON with ticket and category details.
    update_common_fields(
        &mut json_value,
        &ticket,
        &ticket.create_new_id(),
        &ticket.generate_issuance_date(),
        &ticket.generate_expiration_date(8),
    );

    // Update the JSON with category-specific data.
    json_value["outputDescriptor"]["display"]["title"]["text"] = json!(data.title);
    json_value["outputDescriptor"]["display"]["description"]["text"] = json!(data.description);
    json_value["outputDescriptor"]["display"]["subtitle"]["text"] = json!(data.subtitle);
    json_value["outputDescriptor"]["styles"]["hero"]["uri"] = json!(data.hero_uri);
    json_value["issuer"]["styles"]["hero"]["uri"] = json!(data.hero_uri);

    let update_json_string =
        serde_json::to_string(&json_value).expect("Failed to convert JSON value to string");

    // Make a PUT request with the updated JSON
    let request_response = client
        .put(base_url)
        .header("Content-type", "application/json")
        .body(update_json_string.to_owned())
        .send()
        .await;

    match request_response {
        Ok(response) => {
            let response = response.error_for_status()?;
            let response_body: Value = response.json().await?;
            Ok(response_body)
        }
        Err(err) => {
            error!("Error making request: {:?}", err);
            Err(err)
        }
    }
}

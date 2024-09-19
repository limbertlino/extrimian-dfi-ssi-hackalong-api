use crate::rocket;
use crate::models::{Category, Ticket};
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json::Value;
use uuid::Uuid;

fn test_issue_vc_ticket(category: Category) {
    let client = Client::tracked(rocket()).expect("Valid rocket instance");

    let ticket = Ticket {
        name: "alice".to_string(),
        category,
    };

    let body = serde_json::to_string(&ticket).expect("Failede to serialize ticket");

    let response = client
        .put("/issue-vc")
        .header(ContentType::JSON)
        .body(body)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_body: Value = response.into_json().expect("Response was not Json");
    assert!(
        response_body.get("oobContentData").is_some(),
        "oobContentData missing"
    );
    assert!(
        response_body.get("invitationId").is_some(),
        "invitationId missing"
    );

    let invitation_id = response_body["invitationId"].as_str().unwrap();
    assert!(
        Uuid::parse_str(invitation_id).is_ok(),
        "invitationId is not a valid UUID"
    );

    let oob_content = response_body["oobContentData"].as_str().unwrap();
    assert!(
        oob_content.starts_with("didcomm://?_oob="),
        "oobContentData does not have the expected format"
    )
}

#[test]
fn test_issue_vc_standard_ticket() {
    test_issue_vc_ticket(Category::Standard)
}

#[test]
fn test_issue_vc_vip_ticket() {
    test_issue_vc_ticket(Category::Vip)
}

#[test]
fn test_issue_vc_fast_ticket() {
    test_issue_vc_ticket(Category::Fast)
}

#[test]
fn test_issue_vc_extra_ticket() {
    test_issue_vc_ticket(Category::Extra)
}

#[test]
fn test_issue_vc_invalid_data() {
    let client = Client::tracked(rocket()).expect("Valid rocket instance");

    // Casos de datos inválidos
    let cases = vec![
        r#"{"name": "Alice", "category": "InvalidCategory"}"#,
        r#"{"name": "alice", "category": ""}"#,
        r#"{"category": "Standard"}"#,
        r#"{"name": "alice"}"#,
    ];

    for json in cases {
        let response = client
            .put("/issue-vc")
            .header(ContentType::JSON)
            .body(json)
            .dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);

        let response_body: Value = response.into_json().expect("Response was not JSON");
        assert_eq!(response_body["error"], 422);
        assert_eq!(
            response_body["message"],
            "Unprocessable entity: Validation failed"
        );
    }
}

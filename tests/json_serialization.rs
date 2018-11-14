extern crate http_api_problem;
extern crate serde_json;

use http_api_problem::*;

#[test]
fn only_title_serializes_correctly() {
    let problem = HttpApiProblem::new("Error");

    assert_eq_json(&problem, include_str!("responses/new.json"));
}

#[test]
fn with_title_and_type_from_status_serializes_correctly() {
    let problem = HttpApiProblem::with_title_and_type_from_status(500);

    assert_eq_json(&problem, include_str!("responses/with_title_and_type_from_status_500.json"));
}

#[test]
fn with_title_and_type_from_unknown_status_serializes_correctly() {
    let problem = HttpApiProblem::with_title_and_type_from_status(777);

    assert_eq_json(&problem, include_str!("responses/with_title_and_type_from_status_777.json"));
}

#[test]
fn with_title_and_type_from_unknown_client_status_serializes_correctly() {
    let problem = HttpApiProblem::with_title_and_type_from_status(499);

    assert_eq_json(&problem, include_str!("responses/with_title_and_type_from_status_499.json"));
}

#[test]
fn with_title_and_type_from_unknown_server_status_serializes_correctly() {
    let problem = HttpApiProblem::with_title_and_type_from_status(599);

    assert_eq_json(&problem, include_str!("responses/with_title_and_type_from_status_599.json"));
}

#[test]
fn with_title_from_status_serializes_correctly() {
    let problem = HttpApiProblem::with_title_from_status(500);

    assert_eq_json(&problem, include_str!("responses/with_title_from_status_500.json"));
}

#[test]
fn detail_serializes_correctly() {
    let problem = HttpApiProblem::with_title_and_type_from_status(500)
        .set_detail("Unable to connect to the database");

    assert_eq_json(&problem, include_str!("responses/with_title_and_type_from_status_500_with_detail.json"));
}

#[test]
fn additional_fields_serializes_correctly() {
    let mut problem = HttpApiProblem::with_title_and_type_from_status(400)
        .set_detail("A validation error occured");

    problem.set_value("invalid_field", &String::from("first_name")).unwrap();
    problem.set_value("validation_error", &String::from("must be set")).unwrap();

    assert_eq_json(&problem, include_str!("responses/validation_error.json"));
}

fn assert_eq_json(problem: &HttpApiProblem, expected_json: &str) {
    let actual_json = serde_json::to_string_pretty(&problem).expect("failed to serialize");

    assert_eq!(actual_json, expected_json);
}

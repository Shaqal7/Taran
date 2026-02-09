//! Integration tests for taran-core

use taran_config::Scenario;
use taran_core::runner::TestRunner;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_basic_scenario_execution() {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Configure the mock server to respond to GET /test
    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let toml = format!(
        r#"
[scenario]
name = "Integration Test"

[load_profile]
type = "constant"
users = 1
duration = "5s"

[[steps]]
name = "Test Step"
protocol = "http"
method = "GET"
url = "{}/test"

[steps.assertions]
status = 200
"#,
        mock_server.uri()
    );

    let scenario = Scenario::from_str(&toml).expect("Failed to parse scenario");
    scenario.validate().expect("Scenario validation failed");

    let runner = TestRunner::new(scenario);
    let summary = runner.run().await.expect("Test execution failed");

    assert!(summary.total_requests > 0);
    assert_eq!(
        summary.failed_requests, 0,
        "Expected 0 failed requests, got {}",
        summary.failed_requests
    );
    assert_eq!(
        summary.successful_requests, summary.total_requests,
        "All requests should be successful"
    );
}

#[tokio::test]
async fn test_multiple_steps() {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Configure the mock server to respond to GET /get and POST /post
    Mock::given(method("GET"))
        .and(path("/get"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/post"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    let toml = format!(
        r#"
[scenario]
name = "Multi-Step Test"

[load_profile]
type = "constant"
users = 1
duration = "5s"

[[steps]]
name = "GET"
protocol = "http"
method = "GET"
url = "{}/get"

[[steps]]
name = "POST"
protocol = "http"
method = "POST"
url = "{}/post"
body = '{{"key": "value"}}'

[steps.headers]
"Content-Type" = "application/json"
"#,
        mock_server.uri(),
        mock_server.uri()
    );

    let scenario = Scenario::from_str(&toml).expect("Failed to parse scenario");
    let runner = TestRunner::new(scenario);
    let summary = runner.run().await.expect("Test execution failed");

    assert!(summary.total_requests >= 2);
    assert_eq!(summary.failed_requests, 0, "Requests failed");
}

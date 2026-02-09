//! Integration tests for taran-core

use taran_config::Scenario;
use taran_core::runner::TestRunner;

#[tokio::test]
async fn test_basic_scenario_execution() {
    let toml = r#"
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
url = "https://httpbin.org/get"

[steps.assertions]
status = 200
"#;

    let scenario = Scenario::from_str(toml).expect("Failed to parse scenario");
    scenario.validate().expect("Scenario validation failed");

    let runner = TestRunner::new(scenario);
    let summary = runner.run().await.expect("Test execution failed");

    assert!(summary.total_requests > 0);
    assert_eq!(summary.failed_requests, 0);
}

#[tokio::test]
async fn test_multiple_steps() {
    let toml = r#"
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
url = "https://httpbin.org/get"

[[steps]]
name = "POST"
protocol = "http"
method = "POST"
url = "https://httpbin.org/post"
body = '{"key": "value"}'

[steps.headers]
"Content-Type" = "application/json"
"#;

    let scenario = Scenario::from_str(toml).expect("Failed to parse scenario");
    let runner = TestRunner::new(scenario);
    let summary = runner.run().await.expect("Test execution failed");

    assert!(summary.total_requests >= 2);
}

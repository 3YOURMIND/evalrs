use js_sandbox::Script;
use serde::{Deserialize, Serialize};
use serde_json::Value;
// use std::time::Duration;
use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub script: String,
    pub variables: Value,
    pub timeout: Option<u64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub result: Value,
}

#[tokio::main]
async fn main() {
    console_subscriber::init();
    let echo = warp::post()
        .and(warp::path("eval"))
        .and(warp::body::json())
        .map(json_echo);

    warp::serve(echo).run(([127, 0, 0, 1], 3030)).await
}

fn json_echo(request: Request) -> warp::reply::Json {
    let mut script_evaluator = get_script_evaluator(&request.variables); //.with_timeout(Duration::from_millis(1000));

    let result: Value = script_evaluator
        .call(
            "wrapper",
            (
                Value::String(request.script.clone()),
                request.variables.clone(),
            ),
        )
        .unwrap();

    let response = Response { result };
    warp::reply::json(&response)
}

fn get_script_evaluator(variables: &Value) -> Script {
    let arguments: String = match variables {
        Value::Object(object) => object.keys().cloned().collect::<Vec<String>>().join(", "),
        _ => "".to_string(),
    };

    let raw_script = format!(
        r#"function wrapper(script_snippet, {{ {arguments} }} ){{ return eval(script_snippet) }} "#,
        arguments = arguments,
    );

    Script::from_string(&raw_script).unwrap()
}

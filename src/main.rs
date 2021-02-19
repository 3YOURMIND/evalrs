#![feature(proc_macro_hygiene, decl_macro)]
mod components;
use crate::components::payloads::{PricingPayload};
use std::collections::HashMap;
use js_sandbox::{
    Script, 
    AnyError
};
use serde::{Serialize};
use serde_json::{
    json,
    Value,
    error::Error,
};
use rocket::{
    State,
    Data,
    Rocket,
};
use rocket;
use rocket_contrib::json::{JsonValue};


#[derive(Serialize, Debug)]
enum CalcResult {
    Result(Value),
    Error(String),
}

#[derive(Serialize, Debug)]
struct ResultResponse<'a> {
    status: &'a str,
    result: CalcResult,
}

type KeyCache = HashMap<String, String>;


#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::post("/", data = "<data>")]
fn eval(data: Data, _key_cache: State<KeyCache>) -> JsonValue {

    let maybe_json: Result<PricingPayload, Error> 
        = serde_json::de::from_reader(
            data.open()
        );

    let response: ResultResponse = match maybe_json {
        Ok(data)=> {
            let raw_code = &data.script;
            let code = format!(r#"wrapper = (variables) => {{
                   {raw_code}
                }}"#,
                               raw_code = raw_code
            );

            match Script::from_string(&code) {
                Err(error) => ResultResponse { 
                    status: "error", 
                    result: CalcResult::Error(error.to_string()) 
                },
                Ok(mut script) => {
                    let pricing_result: Result<Value, AnyError> 
                        = script.call("wrapper", &data.variables);

                    match pricing_result {
                        Ok(price) => ResultResponse { 
                            status: "ok", 
                            result: CalcResult::Result(price) 
                        },
                        Err(error) => ResultResponse { 
                            status: "error", 
                            result: CalcResult::Error(error.to_string()) 
                        },
                    }
                }
            }
        },
        Err(err)=> {
            ResultResponse { 
                status: "error", 
                result: CalcResult::Error(err.to_string()) 
            }
        },
    };

    JsonValue(json!(response))
}

fn build_rocket() -> Rocket {
    rocket::ignite()
        .mount("/", rocket::routes![index, eval])
        .manage(KeyCache::new())
}

fn main() {
    //#[cfg(feature="static")]
    // uncomment above & build against 
    // musl lib for maximum static links
    build_rocket() 
        .launch();
}

#[cfg(test)]
mod test {
    use super::build_rocket;
    use rocket::local::Client;
    use rocket::http::{
        ContentType, 
        Status
    };

    #[test]
    fn hello_world_on_get() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client.get("/").dispatch();

        assert_eq!(
            response.status(), 
            Status::Ok
        );
        assert_eq!(
            response.body_string(), 
            Some("Hello, world!".into())
        );
    }

    #[test]
    fn eval_payload_on_post() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client
            .post("/")
            .body(r#"{
                "variables":{"A":2,"B":2},
                "script":"return 2;",
                "key":"59bcc3ad6775562f845953cf01624225"
            }"#)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(
            response.status(), 
            Status::Ok
        );
        assert_eq!(
            response.body_string(), 
            Some(r#"{"status":"ok","result":{"Result":2}}"#.to_string())
        );
    }
}


use crate::app_state::AppState;
use crate::cache_backend::CacheBackend;
use crate::errors::EvalrsError;
use crate::request::Request;
use actix_web::web::Data;
use log::{debug, warn};
use serde_json::{json, Value};

pub struct EvaluationOk {
    pub result: Value,
}

pub fn evaluate(request: &Request, data: &mut Data<AppState>) -> Result<EvaluationOk, EvalrsError> {
    let cache = &mut *data.cache.lock().unwrap(); // fixme Learn how to use mutexes!
    let script_code = get_script_from_cache(&request.id, &request.script, cache)?;

    debug!(
        "Trying to evaluate script:\n{}\nwith variables:\n{}",
        &script_code,
        serde_json::to_string(&request.variables).unwrap_or_default()
    );

    let result = serde_json::from_str("10").unwrap();
    Ok(EvaluationOk { result })
}

fn get_script_from_cache<'a>(
    id: &'a Option<String>,
    script: &'a Option<String>,
    cache: &'a mut dyn CacheBackend,
) -> Result<&'a String, EvalrsError> {
    match script {
        Some(script_code) => {
            debug!("Script submitted");
            if let Some(id_value) = id {
                debug!("Saving submitted script with id {}", id_value);
                cache.set(id_value, script_code);
            }

            Ok(script_code)
        }
        None => {
            debug!("Script is not submitted");

            match id {
                Some(id_value) => {
                    debug!("Looking up in cache with id {}", id_value);
                    match cache.get(id_value) {
                        Some(script_code) => {
                            debug!("Script with id {} found", id_value);
                            Ok(script_code)
                        }
                        None => {
                            debug!("Script with id {} not found", id_value);
                            Err(EvalrsError::IdNotFound)
                        }
                    }
                }
                None => {
                    warn!("No script nor id submitted");
                    Err(EvalrsError::NoIdNorScriptSubmitted)
                }
            }
        }
    }
}

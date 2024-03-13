use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalrsError {
    #[error("Variables must be an object")]
    WrongVariablesType,

    #[error("Script not cached and no Id submitted")]
    NoIdNorScriptSubmitted,

    #[error("Script Id not found")]
    IdNotFound,
}

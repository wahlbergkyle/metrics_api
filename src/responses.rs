use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct FailureResponse {
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct DBTextResponse {
    pub response: SuccessResponse,
}

// Implement Display for the Failure response so that we can then implement Error.
impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

// Implement Error for the FailureResponse so that we can `?` (try) the Response
// returned by `lambda_runtime::run(func).await` in `fn main`.
impl std::error::Error for FailureResponse {}

pub type Response = Result<SuccessResponse, FailureResponse>;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SubmitEvaluationRequest {
    pub scores: Vec<ScoreRequest>,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScoreRequest {
    pub criterion_id: Uuid,
    pub score: i32,
}

#[derive(Debug, Serialize)]
pub struct SubmitEvaluationResponse {
    pub status: String,
}

use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AssignmentForEvaluation {
    pub id: Uuid,
    pub status: String,
    pub jury_user_id: Uuid,
    pub round_id: Uuid,
    pub tournament_id: Uuid,
    pub round_status: String,
    pub tournament_status: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EvaluationCriterion {
    pub id: Uuid,
    pub max_score: i32,
}

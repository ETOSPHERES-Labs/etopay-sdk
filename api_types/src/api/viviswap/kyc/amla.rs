use serde::{Deserialize, Serialize};

// data objects

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KycAmlaQuestion {
    /// The unique ID of this question.
    pub id: String,

    /// The question the user has to answer.
    pub question: String,

    /// A list of available answers that the user can choose from.
    pub possible_answers: Vec<String>,

    /// Indicator if this question allows free text answers.
    pub is_free_text: bool,

    /// The minumum number of answers (including the free-text answer) that are required.
    pub min_answers: i32,

    /// The maximum number of answers (including the free-text answer) that are allowed.
    pub max_answers: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AnswerData {
    /// The ID of the question to set the answer to.
    pub question_id: String,

    /// A list of the selected available answers for the question.
    pub answers: Vec<String>,

    /// An optional free-text answer.
    pub freetext_answer: Option<String>,
}

// requests/responses

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetKycAmlaQuestionsResponse {
    pub questions: Vec<KycAmlaQuestion>,
}

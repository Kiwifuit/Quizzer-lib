use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    length: u8,
    items: Vec<QuizItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizItem {
    item_type: QuizItemType,
    question: String,
    answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QuizItemType {
    MultipleChoice,
    FillInTheBlank,
    TrueOrFalse,
}

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

impl From<Vec<QuizItem>> for Quiz {
    fn from(value: Vec<QuizItem>) -> Self {
        Self {
            length: value.len() as u8,
            items: value,
        }
    }
}

impl From<(u8, String, String)> for QuizItem {
    fn from(value: (u8, String, String)) -> Self {
        let (q_type, question, answer) = value;

        Self {
            item_type: QuizItemType::try_from(q_type).unwrap(),
            question,
            answer,
        }
    }
}

#[derive(Debug)]
pub struct InvalidNumberError(u8);

impl TryFrom<u8> for QuizItemType {
    type Error = InvalidNumberError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::MultipleChoice),
            2 => Ok(Self::FillInTheBlank),
            3 => Ok(Self::TrueOrFalse),
            other => Err(InvalidNumberError(other)),
        }
    }
}

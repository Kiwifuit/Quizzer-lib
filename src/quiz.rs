//! Entities for representing quizzes programmatically
//! # Example
//! ```rs
//! let raw_values = vec![
//!   (1u8, "Do you love her?", "yes"),
//!   (1u8, "Amoogns cronut", "no"),
//!   (1u8, "Weeeeeeeeeeee", "yes"),
//!   (1u8, "Fourth question?", "yes"),
//!   (1u8, "Is the next question of the same type?", "no"),
//!   (2u8, "Who is the bestest doggo in the world?", "Rosie"),
//!   (2u8, "Who is the smelliest doggo in the world?", "Tasha"),
//!   (3u8, "What is life?", "Damn bro idk"),
//!   (
//!       3u8,
//!       "What is this hit game called Amongus?",
//!       "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
//!   ),
//! ];
//!
//! let quiz = quizzer::Quiz::from(raw_values)
//! ```
use serde::{Deserialize, Serialize};

/// Represents a quiz
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Quiz {
    length: u8,
    items: Vec<QuizItem>,
}

/// Represents a quiz item, with a type, a question, and an answer
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuizItem {
    item_type: QuizItemType,
    question: String,
    answer: String,
}

/// Represents a quiz type. This can be a Multiple Choice
/// quiz, a Fill in the Blank quiz, or a True or False quiz
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum QuizItemType {
    MultipleChoice,
    FillInTheBlank,
    TrueOrFalse,
}

impl Quiz {
    /// Instantiates a new `Quiz`. No parameters are passed in
    /// as `Quiz` is a lazy-ish struct
    pub fn new() -> Self {
        Self {
            length: 0,
            items: Vec::new(),
        }
    }

    /// Adds a `QuizItem` to this `Quiz`
    pub fn add_quiz(&mut self, quiz: QuizItem) {
        self.length += 1;
        self.items.push(quiz);
    }

    /// Gets the `n`th `Quiz`
    pub fn get_quiz(&self, n: usize) -> Option<&QuizItem> {
        self.items.get(n)
    }

    /// Gets the length of this `Quiz`
    pub const fn get_length(&self) -> u8 {
        self.length
    }
}

impl From<Vec<QuizItem>> for Quiz {
    fn from(value: Vec<QuizItem>) -> Self {
        Self {
            length: value.len() as u8,
            items: value,
        }
    }
}

impl From<Vec<(u8, &str, &str)>> for Quiz {
    fn from(value: Vec<(u8, &str, &str)>) -> Self {
        let items = value
            .iter()
            .map(|i| QuizItem::from(*i))
            .collect::<Vec<QuizItem>>();

        Self {
            length: items.len() as u8,
            items,
        }
    }
}

impl QuizItem {
    /// Creates a new quiz item
    pub fn new(q_type: QuizItemType, question: String, answer: String) -> Self {
        Self {
            item_type: q_type,
            question,
            answer,
        }
    }

    /// Sets the question to this quiz item
    pub fn set_question(&mut self, question: String) {
        self.question = question;
    }

    /// Sets the answer to this quiz item
    pub fn set_answer(&mut self, answer: String) {
        self.answer = answer;
    }

    /// Gets the question to this question
    pub fn get_question(&self) -> &String {
        &self.question
    }

    /// Gets the answer to this question
    pub fn get_answer(&self) -> &String {
        &self.answer
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

impl From<(u8, &str, &str)> for QuizItem {
    fn from(value: (u8, &str, &str)) -> Self {
        let (q_type, question, answer) = value;

        Self {
            item_type: QuizItemType::try_from(q_type).unwrap(),
            question: question.to_string(),
            answer: answer.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct InvalidNumberError(u8);

impl TryFrom<u8> for QuizItemType {
    type Error = InvalidNumberError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::TrueOrFalse),
            2 => Ok(Self::MultipleChoice),
            3 => Ok(Self::FillInTheBlank),
            other => Err(InvalidNumberError(other)),
        }
    }
}

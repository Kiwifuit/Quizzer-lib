pub struct Quiz {
    length: u8,
    items: Vec<QuizItem>,
}

pub struct QuizItem {
    item_type: QuizItemType,
    question: String,
    answer: String,
}

pub enum QuizItemType {
    MultipleChoice,
    FillInTheBlank,
    TrueOrFalse,
}

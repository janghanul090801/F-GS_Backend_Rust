#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionAnswer {
    pub id: i32,
    pub form_answer_id: i32,
    pub question_id: i32,
    pub answer: String,
}

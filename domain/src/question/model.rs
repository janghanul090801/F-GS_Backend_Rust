#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub id: i32,
    pub question_type: String,
    pub label: String,
    pub required: bool,
    pub is_file: bool,
    pub form_id: i32,
}

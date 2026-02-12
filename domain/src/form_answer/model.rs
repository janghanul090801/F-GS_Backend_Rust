#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormAnswer {
    pub id: i32,
    pub form_id: i32,
    pub answerer_id: i32,
}

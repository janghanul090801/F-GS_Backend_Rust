#[derive(Debug, Clone, PartialEq)]
pub struct Review {
    pub id: i32,
    pub score: f32,
    pub content: String,
    pub user_id: i32,
    pub target_user_id: i32,
}

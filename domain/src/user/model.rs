use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub student_id: i32,
    pub self_introduction: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub portfolio_path: String,
    pub profile_path: String,
    pub rating: f64,
    pub is_active: bool,
    pub is_admin: bool,
    pub is_will_deleted: bool,
}

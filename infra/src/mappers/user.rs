use domain::user::model::User;
use crate::seaorm::entity::user::Model;

impl From<Model> for User {
    fn from(value: Model) -> Self {
        User {
            id: value.id,
            name: value.name.clone(),
            student_id: value.student_id,
            self_introduction: value.self_introduction.clone(),
            email: value.email.clone(),
            password: value.password.clone(),
            created_at: value.created_at,
            portfolio_path: value.portfolio_path.clone(),
            profile_path: value.profile_path.clone(),
            rating: value.rating,
            is_active: value.is_active,
            is_admin: value.is_admin,
            is_will_deleted: value.is_will_deleted,
        }
    }
}

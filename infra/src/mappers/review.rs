use domain::review::model::Review;
use crate::seaorm::entity::review::Model;

impl From<Model> for Review {
    fn from(value: Model) -> Self {
        Review {
            id: value.id,
            score: value.score,
            content: value.content.clone(),
            user_id: value.user_id,
            target_user_id: value.target_user_id,
        }
    }
}

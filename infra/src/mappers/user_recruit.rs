use domain::user_recruit::model::UserRecruit;
use crate::seaorm::entity::user_recruit::Model;

impl From<Model> for UserRecruit {
    fn from(value: Model) -> Self {
        UserRecruit {
            post_id: value.post_id,
            user_id: value.user_id,
        }
    }
}

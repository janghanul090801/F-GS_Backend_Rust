use domain::user_tag::model::UserTag;
use crate::seaorm::entity::user_tag::Model;

impl From<Model> for UserTag {
    fn from(value: Model) -> Self {
        UserTag {
            user_id: value.user_id,
            tag_id: value.tag_id,
        }
    }
}

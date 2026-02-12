use domain::post_tag::model::PostTag;
use crate::seaorm::entity::post_tag::Model;

impl From<Model> for PostTag {
    fn from(value: Model) -> Self {
        PostTag {
            post_id: value.post_id,
            tag_id: value.tag_id,
        }
    }
}

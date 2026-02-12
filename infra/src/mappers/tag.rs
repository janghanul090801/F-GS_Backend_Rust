use domain::tag::model::Tag;
use crate::seaorm::entity::tag::Model;

impl From<Model> for Tag {
    fn from(value: Model) -> Self {
        Tag {
            id: value.id,
            name: value.name.clone(),
            bg_color: value.bg_color.clone(),
            font_color: value.font_color.clone(),
            usage: value.usage,
            tag_type: value.tag_type.clone(),
        }
    }
}

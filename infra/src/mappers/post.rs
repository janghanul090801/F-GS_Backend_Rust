use domain::post::model::Post;
use crate::seaorm::entity::post::Model;

impl From<Model> for Post {
    fn from(value: Model) -> Self {
        Post {
            id: value.id,
            title: value.title.clone(),
            content: value.content.clone(),
            category: value.category.clone(),
            max_recruits: value.max_recruits,
            current_recruits: value.current_recruits,
            is_finished: value.is_finished,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deadline: value.deadline,
            author_id: value.author_id,
            attachments: value.attachments.clone(),
            apply: value.apply,
            is_new: value.is_new,
        }
    }
}

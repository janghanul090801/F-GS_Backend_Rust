use domain::form::model::Form;
use crate::seaorm::entity::form::Model;


impl From<Model> for Form {
    fn from(value: Model) -> Self {
        Form {
            id: value.id,
            post_id: value.post_id,
        }
    }
}
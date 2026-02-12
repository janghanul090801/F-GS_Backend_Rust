use domain::form_answer::model::FormAnswer;
use crate::seaorm::entity::form_answer::Model;

impl From<Model> for FormAnswer {
    fn from(value: Model) -> Self {
        FormAnswer {
            id: value.id,
            form_id: value.form_id,
            answerer_id: value.answerer_id,
        }
    }
}

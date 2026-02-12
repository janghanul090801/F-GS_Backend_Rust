use domain::question::model::Question;
use crate::seaorm::entity::question::Model;

impl From<Model> for Question {
    fn from(value: Model) -> Self {
        Question {
            id: value.id,
            question_type: value.question_type.clone(),
            label: value.label.clone(),
            required: value.required,
            is_file: value.is_file,
            form_id: value.form_id,
        }
    }
}

use domain::question_answer::model::QuestionAnswer;
use crate::seaorm::entity::question_answer::Model;

impl From<Model> for QuestionAnswer {
    fn from(value: Model) -> Self {
        QuestionAnswer {
            id: value.id,
            form_answer_id: value.form_answer_id,
            question_id: value.question_id,
            answer: value.answer.clone(),
        }
    }
}

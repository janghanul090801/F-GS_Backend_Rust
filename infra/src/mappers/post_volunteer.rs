use domain::post_volunteer::model::PostVolunteer;
use crate::seaorm::entity::post_volunteer::Model;

impl From<Model> for PostVolunteer {
    fn from(value: Model) -> Self {
        PostVolunteer {
            post_id: value.post_id,
            user_id: value.user_id,
        }
    }
}

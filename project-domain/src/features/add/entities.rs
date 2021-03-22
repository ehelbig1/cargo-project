#[derive(Debug, PartialEq)]
pub struct UpdateSuccess(String);

impl UpdateSuccess {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn from_model(model: project_data::features::add::models::UpdateSuccess) -> Self {
        let message = model.get_message();
        Self::new(message)
    }
}

#[derive(Debug, PartialEq)]
pub struct UpdateError(String);

impl UpdateError {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn from_err(error: project_data::features::add::models::UpdateError) -> Self {
        let message = error.get_message();
        Self::new(message)
    }
}

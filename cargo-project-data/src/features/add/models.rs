#[derive(Debug, PartialEq)]
pub struct UpdateSuccess(String);

impl UpdateSuccess {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn get_message(self) -> String {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct UpdateError(String);

impl UpdateError {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn get_message(self) -> String {
        self.0
    }
}

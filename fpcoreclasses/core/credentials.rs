// core/credentials.rs
#[derive(Debug, Default)]
pub struct Credentials {
    /// Operator Name or Operator ID.
    pub operator: String,

    /// Operator Password.
    pub operator_password: String,
}

impl Credentials {
    pub fn new(operator: String, operator_password: String) -> Self {
        Self {
            operator,
            operator_password,
        }
    }
}

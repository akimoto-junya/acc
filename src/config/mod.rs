use serde::{Deserialize, Serialize};

pub use user::User;
pub use init::Init;
pub use submit::Submit;
pub use test::Test;

pub mod user;
pub mod init;
pub mod submit;
pub mod test;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub user: User,
    pub init: Init,
    pub submit: Submit,
    pub test: Test,
}

pub trait Overridable {
    fn override_by_default(&mut self);
}

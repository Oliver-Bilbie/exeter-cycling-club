use bounce::prelude::Atom;
use crate::helpers::handle_auth::UserData;

#[derive(Atom, PartialEq, Default)]
pub struct AuthState {
    pub user_data: Option<UserData>,
}

use crate::helpers::handle_auth::UserData;
use bounce::prelude::Atom;

#[derive(Atom, PartialEq, Default, Debug)]
pub struct AuthState {
    pub user_data: Option<UserData>,
}

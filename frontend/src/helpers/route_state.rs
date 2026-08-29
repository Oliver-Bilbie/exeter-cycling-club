use bounce::prelude::Atom;

use crate::helpers::get_route::RouteStatus;

#[derive(Atom, PartialEq, Debug)]
pub struct RouteState {
    pub status: RouteStatus,
}

impl Default for RouteState {
    fn default() -> Self {
        Self {
            status: RouteStatus::Loading,
        }
    }
}

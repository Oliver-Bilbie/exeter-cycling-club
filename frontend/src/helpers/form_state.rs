#[derive(PartialEq, Clone)]
pub enum FormState {
    Ready,
    Loading,
    Complete,
}

#[derive(PartialEq, Clone)]
pub enum RequestState {
    Loading,
    Success,
    Failure,
}

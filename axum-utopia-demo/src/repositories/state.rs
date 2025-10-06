use crate::repositories::user::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: UserRepository,
}

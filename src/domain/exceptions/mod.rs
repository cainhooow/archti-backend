#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Generic(String),
}

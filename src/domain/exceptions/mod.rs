#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Generic(String),
}

#[derive(Debug)]
pub enum DomainError {
    InvalidInput,
    Generic(String),
}

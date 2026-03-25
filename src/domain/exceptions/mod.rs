#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Generic(String),
}

#[derive(Debug)]
pub enum MailerError {
    Generic(String),
}

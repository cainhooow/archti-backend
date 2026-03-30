use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub struct CreateUserInput {}

pub struct CreateUserCommand {
    pub db: Arc<DatabaseConnection>,
}

impl CreateUserCommand {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn handle(&self, input: CreateUserCommand) {
        
    }
}

use std::sync::Arc;

use crate::infrastructure::bootstrap::AppContainer;

#[derive(Clone)]
pub struct HttpState {
    pub app: Arc<AppContainer>,
}

impl HttpState {
    pub fn new(app: Arc<AppContainer>) -> Self {
        Self { app }
    }
}

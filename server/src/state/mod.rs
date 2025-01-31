use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub document_resolver: Arc<RwLock<crate::document_resolver::DocumentResolver>>,
}

use actix_web::web::Bytes;
pub mod file_system;
use super::ResultType;

pub enum GetFileResponse {
    Redirect(String),
    Bytes(Bytes),
}

#[async_trait::async_trait]
pub trait FileStorageBackend {
    async fn remove_file(&self, token: &str) -> ResultType<()>;
    async fn save_file(&self, token: &str, data: &[u8]) -> ResultType<()>;
    async fn get_file(&self, token: &str) -> ResultType<GetFileResponse>;
}

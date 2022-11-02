use std::path::{Path, PathBuf};

use actix_web::web::Bytes;
use tokio::io::AsyncWriteExt;

use crate::core::ResultType;

use super::{FileStorageBackend, GetFileResponse};
use anyhow::anyhow;
pub struct FileSystemStorageBackend {
    upload_dir: PathBuf,
}

impl FileSystemStorageBackend {
    pub fn new(upload_dir: &Path) -> std::io::Result<Self> {
        if !upload_dir.exists() {
            std::fs::create_dir(upload_dir)?;
        }
        Ok(Self {
            upload_dir: upload_dir.to_path_buf(),
        })
    }
}

#[async_trait::async_trait]
impl FileStorageBackend for FileSystemStorageBackend {
    async fn remove_file(&self, token: &str) -> ResultType<()> {
        tokio::fs::remove_file(self.upload_dir.join(token))
            .await
            .map_err(|e| e.into())
    }
    async fn save_file(&self, token: &str, data: &[u8]) -> ResultType<()> {
        let mut file = tokio::fs::File::create(self.upload_dir.join(token))
            .await
            .map_err(|e| anyhow!("Failed to create file {}: {}", token, e))?;
        file.write_all(data).await?;
        return Ok(());
    }
    async fn get_file(&self, token: &str) -> ResultType<GetFileResponse> {
        let file = tokio::fs::read(self.upload_dir.join(token))
            .await
            .map_err(|e| anyhow!("Failed to read file {}: {}", token, e))?;
        return Ok(GetFileResponse::Bytes(Bytes::from(file)));
    }
}

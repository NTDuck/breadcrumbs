use ::async_trait::async_trait;

use crate::utils::aliases;
use crate::utils::pagination::PaginationRequest;
use crate::utils::pagination::PaginationResponse;

// Possible impls: v1 (/snowflake?), v7, and perhaps more :3
#[async_trait]
pub trait UuidGenerator {
    async fn generate(self: ::std::sync::Arc<Self>) -> aliases::result::Fallible<::domain::Uuid>;

    async fn get_timestamp(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> aliases::result::Fallible<aliases::time::Timestamp>;
}

#[async_trait]
pub trait UuidFormatter {
    async fn format(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> aliases::result::Fallible<aliases::string::String>;
}

#[async_trait]
pub trait TaskRepository {
    async fn save(self: ::std::sync::Arc<Self>, task: ::domain::Task) -> aliases::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, task_id: ::domain::Uuid) -> aliases::result::Fallible;

    async fn show(self: ::std::sync::Arc<Self>, pagination_request: PaginationRequest) -> aliases::result::Fallible<PaginationResponse<::domain::Task>>;
}

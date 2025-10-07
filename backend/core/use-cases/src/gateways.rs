use ::async_trait::async_trait;
use ::domain::Task;
use ::domain::Uuid;

use crate::utils::aliases;
use crate::utils::pagination::PaginationRequest;
use crate::utils::pagination::PaginationResponse;

// Possible impls: v1 (/snowflake?), v7, and perhaps more :3
#[async_trait]
pub trait UuidGenerator {
    async fn generate(self: ::std::sync::Arc<Self>) -> aliases::result::Fallible<Uuid>;
}

#[async_trait]
pub trait TaskRepository {
    async fn save(self: ::std::sync::Arc<Self>, task: Task) -> aliases::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, task_id: Uuid) -> aliases::result::Fallible;

    async fn show(self: ::std::sync::Arc<Self>, pagination_request: PaginationRequest) -> aliases::result::Fallible<PaginationResponse<Task>>;
}

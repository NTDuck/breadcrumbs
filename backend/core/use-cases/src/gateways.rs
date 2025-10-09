use ::async_trait::async_trait;

use crate::utils::pagination::PaginationRequest;
use crate::utils::pagination::PaginationResponse;

#[async_trait]
pub trait UuidGenerator: ::core::marker::Send + ::core::marker::Sync {
    async fn generate(self: ::std::sync::Arc<Self>) -> ::aliases::result::Fallible<::domain::Uuid>;

    async fn get_timestamp(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> ::aliases::result::Fallible<::aliases::time::Timestamp>;
}

#[async_trait]
pub trait UuidFormatter: ::core::marker::Send + ::core::marker::Sync {
    async fn format(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> ::aliases::result::Fallible<::aliases::string::String>;
}

#[async_trait]
pub trait TaskRepository: ::core::marker::Send + ::core::marker::Sync {
    async fn save(self: ::std::sync::Arc<Self>, task: ::domain::Task) -> ::aliases::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, task_id: ::domain::Uuid) -> ::aliases::result::Fallible;

    async fn show(self: ::std::sync::Arc<Self>, pagination_request: PaginationRequest) -> ::aliases::result::Fallible<PaginationResponse<::domain::Task>>;
}

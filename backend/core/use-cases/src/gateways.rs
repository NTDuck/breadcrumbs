use ::async_trait::async_trait;

use crate::boundaries::models::pagination::*;

#[async_trait]
pub trait UuidGenerator {
    async fn generate(self: ::std::sync::Arc<Self>) -> ::aliases::result::Fallible<::domain::Uuid>;

    async fn get_timestamp(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> ::aliases::result::Fallible<::aliases::time::Timestamp>;
}

#[async_trait]
pub trait UuidCodec {
    async fn format(self: ::std::sync::Arc<Self>, uuid: ::domain::Uuid) -> ::aliases::result::Fallible<::aliases::string::String>;

    async fn parse(self: ::std::sync::Arc<Self>, uuid: ::aliases::string::String) -> ::aliases::result::Fallible<::domain::Uuid>;
}

#[async_trait]
pub trait TaskRepository {
    async fn save(self: ::std::sync::Arc<Self>, task: ::domain::Task) -> ::aliases::result::Fallible;
    async fn remove(self: ::std::sync::Arc<Self>, task_id: ::domain::Uuid) -> ::aliases::result::Fallible;

    async fn show(self: ::std::sync::Arc<Self>, pagination_request: PaginationRequest) -> ::aliases::result::Fallible<PaginationResponse<::domain::Task>>;

    async fn contains(self: ::std::sync::Arc<Self>, task_id: ::domain::Uuid) -> ::aliases::result::Fallible<bool> {
        let tasks = ::std::sync::Arc::clone(&self).show(PaginationRequest::unbounded()).await?.items;

        ::aliases::result::Fallible::Ok(tasks.into_iter()
            .any(|task| task.id == task_id))
    }
}

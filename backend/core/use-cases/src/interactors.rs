use ::async_trait::async_trait;
use crate::boundaries::*;
use crate::gateways::*;
use crate::utils::aliases;

#[derive(::bon::Builder)]
pub struct CreateTaskInteractor {
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateTaskBoundary for CreateTaskInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: CreateTaskRequest) -> aliases::result::Fallible<CreateTaskResponse> {
        let task_description = ::domain::TaskDescription::builder()
            .value(request.task_description)
            .build();

        let task_description = match task_description {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(error) => return aliases::result::Fallible::Ok(CreateTaskResponse::Err(error.into())),
        };

        let task_id = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

        let task = ::domain::Task::builder()
            .id(task_id)
            .description(task_description)
            .status(::domain::TaskStatus::Pending)
            .build();

        ::std::sync::Arc::clone(&self.task_repository).save(task).await?;

        aliases::result::Fallible::Ok(CreateTaskResponse::Ok(()))
    }
}

#[derive(::bon::Builder)]
pub struct ViewTasksInteractor {
    task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,

    task_assembler: ::std::sync::Arc<models::TaskAssembler>,
}

#[async_trait]
impl ViewTasksBoundary for ViewTasksInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewTasksRequest) -> aliases::result::Fallible<ViewTasksResponse> {
        let tasks = ::std::sync::Arc::clone(&self.task_repository).show(request.pagination_request).await?;
        let tasks = tasks.map(|task| ::std::sync::Arc::clone(&self.task_assembler).assemble(task)).await?;

        aliases::result::Fallible::Ok(ViewTasksResponse::builder().tasks(tasks).build())
    }
}

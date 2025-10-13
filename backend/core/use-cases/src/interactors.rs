use ::async_trait::async_trait;
use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateTaskInteractor {
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
    task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl CreateTaskBoundary for CreateTaskInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: CreateTaskRequest) -> ::aliases::result::Fallible<CreateTaskResponse> {
        let task_description = ::domain::TaskDescription::builder()
            .value(request.task_description)
            .build();

        let task_description = match task_description {
            ::core::result::Result::Ok(value) => value,
            ::core::result::Result::Err(error) => return ::aliases::result::Fallible::Ok(CreateTaskResponse::Err(error.into())),
        };

        let task_id = loop {
            let uuid = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

            if !::std::sync::Arc::clone(&self.task_repository).contains(uuid).await? {
                break uuid;
            }
        };

        let task = ::domain::Task::builder()
            .id(task_id)
            .description(task_description)
            .status(::domain::TaskStatus::Pending)
            .build();

        ::std::sync::Arc::clone(&self.task_repository).save(task).await?;

        ::aliases::result::Fallible::Ok(CreateTaskResponse::Ok(()))
    }
}

#[derive(::bon::Builder)]
pub struct RemoveTaskInteractor {
    uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
    task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,
}

#[async_trait]
impl RemoveTaskBoundary for RemoveTaskInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: RemoveTaskRequest) -> ::aliases::result::Fallible<RemoveTaskResponse> {
        let task_id = ::std::sync::Arc::clone(&self.uuid_codec).parse(request.task_id.clone()).await?;

        if !::std::sync::Arc::clone(&self.task_repository).contains(task_id).await? {
            return ::aliases::result::Fallible::Ok(RemoveTaskResponse::Err(RemoveTaskErrResponse::TaskNotFound { task_id: request.task_id }));
        }

        ::std::sync::Arc::clone(&self.task_repository).remove(task_id).await?;

        ::aliases::result::Fallible::Ok(RemoveTaskResponse::Ok(()))
    }
}

pub struct ViewTasksInteractor {
    task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,

    task_assembler: ::std::sync::Arc<models::TaskAssembler>,
}

#[::bon::bon]
impl ViewTasksInteractor {
    #[builder(builder_type(vis = "pub"))]
    fn new(
        uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
        uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
        task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,
    ) -> Self {
        Self {
            task_repository,
            task_assembler: ::std::sync::Arc::new(models::TaskAssembler::builder()
                .uuid_generator(::std::sync::Arc::clone(&uuid_generator))
                .uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                .build()),
        }
    }
}

#[async_trait]
impl ViewTasksBoundary for ViewTasksInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewTasksRequest) -> ::aliases::result::Fallible<ViewTasksResponse> {
        let tasks = ::std::sync::Arc::clone(&self.task_repository).show(request.pagination_request).await?;
        let tasks = tasks.map(|task| ::std::sync::Arc::clone(&self.task_assembler).assemble(task)).await?;

        ::aliases::result::Fallible::Ok(ViewTasksResponse::builder()
            .pagination_response(tasks)
            .build())
    }
}

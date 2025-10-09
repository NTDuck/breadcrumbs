use ::async_trait::async_trait;
use crate::boundaries::*;
use crate::gateways::*;

#[derive(::bon::Builder)]
pub struct CreateTaskInteractor {
    uuid_generator: ::std::sync::Arc<dyn UuidGenerator>,
    task_repository: ::std::sync::Arc<dyn TaskRepository>,
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

        let task_id = ::std::sync::Arc::clone(&self.uuid_generator).generate().await?;

        let task = ::domain::Task::builder()
            .id(task_id)
            .description(task_description)
            .status(::domain::TaskStatus::Pending)
            .build();

        ::std::sync::Arc::clone(&self.task_repository).save(task).await?;

        ::aliases::result::Fallible::Ok(CreateTaskResponse::Ok(()))
    }
}

pub struct ViewTasksInteractor {
    task_repository: ::std::sync::Arc<dyn TaskRepository>,

    task_assembler: ::std::sync::Arc<models::TaskAssembler>,
}

#[::bon::bon]
impl ViewTasksInteractor {
    #[builder(builder_type(vis = "pub"))]
    fn new(
        uuid_generator: ::std::sync::Arc<dyn UuidGenerator>,
        uuid_formatter: ::std::sync::Arc<dyn UuidFormatter>,
        task_repository: ::std::sync::Arc<dyn TaskRepository>,
    ) -> Self {
        Self {
            task_repository,
            task_assembler: ::std::sync::Arc::new(models::TaskAssembler::builder()
                .uuid_generator(::std::sync::Arc::clone(&uuid_generator))
                .uuid_formatter(::std::sync::Arc::clone(&uuid_formatter))
                .build()),
        }
    }
}

#[async_trait]
impl ViewTasksBoundary for ViewTasksInteractor {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewTasksRequest) -> ::aliases::result::Fallible<ViewTasksResponse> {
        let tasks = ::std::sync::Arc::clone(&self.task_repository).show(request.pagination_request).await?;
        let tasks = tasks.map(|task| ::std::sync::Arc::clone(&self.task_assembler).assemble(task)).await?;

        ::aliases::result::Fallible::Ok(ViewTasksResponse::builder().tasks(tasks).build())
    }
}

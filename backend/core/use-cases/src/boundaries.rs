use ::async_trait::async_trait;

use crate::utils::{pagination::{PaginationRequest, PaginationResponse}};

#[async_trait]
pub trait CreateTaskBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: CreateTaskRequest) -> ::aliases::result::Fallible<CreateTaskResponse>;
}

#[derive(::bon::Builder)]
#[builder(on(::aliases::string::String, into))]
pub struct CreateTaskRequest {
    pub task_description: ::aliases::string::String,
}

pub type CreateTaskResponse = ::core::result::Result<CreateTaskOkResponse, CreateTaskErrResponse>;

pub type CreateTaskOkResponse = ();

pub enum CreateTaskErrResponse {
    TaskDescriptionLengthUnderflow {
        actual: usize,
        min_expected: usize,
    },
    TaskDescriptionLengthOverflow {
        actual: usize,
        max_expected: usize,
    },
}

impl ::core::convert::From<::domain::TaskDescriptionError> for CreateTaskErrResponse {
    fn from(error: ::domain::TaskDescriptionError) -> Self {
        match error {
            ::domain::TaskDescriptionError::LengthUnderflow { actual, min_expected } =>
                CreateTaskErrResponse::TaskDescriptionLengthUnderflow { actual, min_expected },
            ::domain::TaskDescriptionError::LengthOverflow { actual, max_expected } =>
                CreateTaskErrResponse::TaskDescriptionLengthOverflow { actual, max_expected },
        }
    }
}

#[async_trait]
pub trait ViewTasksBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewTasksRequest) -> ::aliases::result::Fallible<ViewTasksResponse>;
}

#[derive(::bon::Builder)]
pub struct ViewTasksRequest {
    pub pagination_request: PaginationRequest,
}

#[derive(::bon::Builder)]
pub struct ViewTasksResponse {
    pub tasks: PaginationResponse<self::models::Task>,
}

pub mod models {
    use crate::{gateways::{UuidFormatter, UuidGenerator}};

    #[derive(::bon::Builder)]
    #[builder(on(::aliases::string::String, into))]
    pub struct Task {
        pub id: ::aliases::string::String,
        pub description: ::aliases::string::String,
        pub status: TaskStatus,

        pub created_at: ::aliases::time::Timestamp,
    }

    #[derive(::bon::Builder)]
    pub(crate) struct TaskAssembler {
        uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
        uuid_formatter: ::std::sync::Arc<dyn UuidFormatter + ::core::marker::Send + ::core::marker::Sync>,
    }

    impl TaskAssembler {
        pub(crate) async fn assemble(self: ::std::sync::Arc<Self>, task: ::domain::Task) -> ::aliases::result::Fallible<Task> {
            let task = Task::builder()
                .id(::std::sync::Arc::clone(&self.uuid_formatter).format(&task.id).await?)
                .description(task.description.to_string())
                .status(task.status.into())
                .created_at(::std::sync::Arc::clone(&self.uuid_generator).get_timestamp(&task.id).await?)
                .build();

            ::aliases::result::Fallible::Ok(task)
        }
    }

    pub enum TaskStatus {
        Pending,
        InProgress,
        Completed,
    }

    impl ::core::convert::From<::domain::TaskStatus> for TaskStatus {
        fn from(status: ::domain::TaskStatus) -> Self {
            match status {
                ::domain::TaskStatus::Pending => TaskStatus::Pending,
                ::domain::TaskStatus::InProgress => TaskStatus::InProgress,
                ::domain::TaskStatus::Completed => TaskStatus::Completed,
            }
        }
    }
}

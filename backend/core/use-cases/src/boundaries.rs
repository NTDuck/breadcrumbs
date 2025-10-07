use ::async_trait::async_trait;
use ::domain::TaskDescriptionError;

use crate::utils::{pagination::{PaginationRequest, PaginationResponse}};
use crate::utils::aliases;

#[async_trait]
pub trait CreateTaskBoundary: ::core::marker::Send + ::core::marker::Sync {
    async fn apply(self: ::std::sync::Arc<Self>, request: CreateTaskRequest) -> aliases::result::Fallible<CreateTaskResponse>;
}

#[derive(::bon::Builder)]
#[builder(on(_, into))]
pub struct CreateTaskRequest {
    pub task_description: ::std::borrow::Cow<'static, str>,
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

impl ::core::convert::From<TaskDescriptionError> for CreateTaskErrResponse {
    fn from(error: TaskDescriptionError) -> Self {
        match error {
            TaskDescriptionError::LengthUnderflow { actual, min_expected } =>
                CreateTaskErrResponse::TaskDescriptionLengthUnderflow { actual, min_expected },
            TaskDescriptionError::LengthOverflow { actual, max_expected } =>
                CreateTaskErrResponse::TaskDescriptionLengthOverflow { actual, max_expected },
        }
    }
}

#[async_trait]
pub trait ViewTasksBoundary: ::core::marker::Send + ::core::marker::Sync {
    async fn apply(self: ::std::sync::Arc<Self>, request: ViewTasksRequest) -> aliases::result::Fallible<ViewTasksResponse>;
}

#[derive(::bon::Builder)]
#[builder(on(_, into))]
pub struct ViewTasksRequest {
    pub pagination_request: PaginationRequest,
}

pub struct ViewTasksResponse {
    pub pagination_response: PaginationResponse<self::models::Task>,
}

pub mod models {
    #[derive(::bon::Builder)]
    #[builder(on(::std::borrow::Cow<'_, str>, into))]
    pub struct Task {
        pub id: ::std::borrow::Cow<'static, str>,
        pub description: ::std::borrow::Cow<'static, str>,
        pub status: TaskStatus,

        pub created_at: ::chrono::NaiveDateTime,
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

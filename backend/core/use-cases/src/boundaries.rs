use ::async_trait::async_trait;

#[async_trait]
pub trait CreateTaskBoundary {
    async fn apply(self: ::std::sync::Arc<Self>, request: CreateTaskRequest) -> ::aliases::result::Fallible<CreateTaskResponse>;
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
#[cfg_attr(feature = "bon", builder(on(::aliases::string::String, into)))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct CreateTaskRequest {
    pub task_description: ::aliases::string::String,
}

pub type CreateTaskResponse = ::core::result::Result<CreateTaskOkResponse, CreateTaskErrResponse>;

pub type CreateTaskOkResponse = ();

#[derive(::core::fmt::Debug, ::thiserror::Error)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case", rename_all_fields = "kebab-case"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub enum CreateTaskErrResponse {
    #[error("Task description is too short (expected minimum {min_expected}, found {actual})")]
    TaskDescriptionLengthUnderflow {
        actual: usize,
        min_expected: usize,
    },

    #[error("Task description is too long (expected maximum {max_expected}, found {actual})")]
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

#[derive(::core::fmt::Debug, ::bon::Builder)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewTasksRequest {
    pub pagination_request: models::pagination::PaginationRequest,
}

#[derive(::core::fmt::Debug, ::bon::Builder)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
#[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
pub struct ViewTasksResponse {
    pub tasks: models::pagination::PaginationResponse<self::models::Task>,
}

pub mod models {
    use crate::{gateways::{UuidFormatter, UuidGenerator}};

    #[derive(::core::fmt::Debug, ::core::clone::Clone)]
    #[derive(::bon::Builder)]
    #[builder(on(::aliases::string::String, into))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
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

    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case", rename_all_fields = "kebab-case"))]
    #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
    #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
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

    pub mod pagination {
        use ::futures::prelude::*;

        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
        #[derive(::bon::Builder)]
        #[builder(const)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
        #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
        #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
        pub struct PaginationRequest {
            pub page_number: usize,
            pub max_page_size: usize,
        }

        impl PaginationRequest {
            pub const fn unbounded() -> Self {
                Self::builder()
                    .page_number(MIN_PAGE_NUMBER)
                    .max_page_size(::core::usize::MAX)
                    .build()
            }
        }

        #[derive(::core::fmt::Debug, ::core::clone::Clone)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
        #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
        #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
        pub struct PaginationResponse<Item> {
            pub items: ::std::vec::Vec<Item>,

            pub page_size: usize,
            pub max_page_size: usize,
            pub page_number: usize,
            pub max_page_number: usize,
        }

        #[::bon::bon]
        impl<Item> PaginationResponse<Item> {
            #[builder(builder_type(vis = "pub"))]
            fn new(
                items: ::std::vec::Vec<Item>,
                request: PaginationRequest,
                total: usize,
            ) -> Self {
                let page_size = items.len();

                let PaginationRequest { page_number, max_page_size } = request;

                let max_page_number = match total {
                    0 => MIN_PAGE_NUMBER,
                    _ => total.div_ceil(max_page_size),
                };

                Self {
                    items,

                    page_size,
                    max_page_size,
                    page_number,
                    max_page_number,
                }
            }
        }

        impl<Item> PaginationResponse<Item> {
            pub async fn map<Mapper, Future, MappedItem>(self, mut mapper: Mapper) -> ::aliases::result::Fallible<PaginationResponse<MappedItem>>
            where
                Mapper: ::core::ops::FnMut(Item) -> Future,
                Future: ::core::future::Future<Output = ::aliases::result::Fallible<MappedItem>>,
            {
                let items = ::futures::stream::iter(self.items)
                    .then(|item| (mapper)(item))
                    .try_collect::<::std::vec::Vec<_>>()
                    .await?;

                let Self { page_size, max_page_size, page_number, max_page_number, .. } = self;

                ::aliases::result::Fallible::Ok(PaginationResponse {
                    items,

                    page_size,
                    max_page_size,
                    page_number,
                    max_page_number,
                })
            }
        }

        impl<Item> ::core::ops::Deref for PaginationResponse<Item> {
            type Target = ::std::vec::Vec<Item>;

            fn deref(&self) -> &::std::vec::Vec<Item> {
                &self.items
            }
        }

        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy)]
        #[derive(::bon::Builder)]
        #[builder(const)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
        #[cfg_attr(feature = "wasm-bindings", derive(::tsify::Tsify))]
        #[cfg_attr(feature = "wasm-bindings", tsify(from_wasm_abi, into_wasm_abi))]
        pub struct PaginationRange {
            pub offset: usize,
            pub limit: usize,
        }

        impl ::core::convert::From<PaginationRange> for ::core::ops::Range<usize> {
            fn from(range: PaginationRange) -> Self {
                range.offset .. range.offset + range.limit
            }
        }

        impl ::core::convert::From<PaginationRequest> for PaginationRange {
            fn from(request: PaginationRequest) -> Self {
                let PaginationRequest { page_number, max_page_size } = request;

                let offset = page_number
                    .saturating_sub(MIN_PAGE_NUMBER)
                    .saturating_mul(max_page_size);
                let limit = max_page_size;

                Self::builder()
                    .offset(offset)
                    .limit(limit)
                    .build()
            }
        }

        const MIN_PAGE_NUMBER: usize = 1;
    }
}

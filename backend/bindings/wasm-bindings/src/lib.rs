use ::use_cases::gateways::*;
use ::use_cases::boundaries::*;
use ::use_cases::interactors::*;
use ::infrastructures::*;
use ::wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(::bon::Builder)]
#[builder(builder_type(vis = "pub(self)"))]
pub struct Application {
    #[wasm_bindgen(skip)]
    create_task_boundary: ::std::sync::Arc<dyn CreateTaskBoundary + ::core::marker::Send + ::core::marker::Sync>,
    #[wasm_bindgen(skip)]
    view_tasks_boundary: ::std::sync::Arc<dyn ViewTasksBoundary + ::core::marker::Send + ::core::marker::Sync>,
}

#[wasm_bindgen]
#[::bon::bon]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::new_from_gateways()
            .uuid_generator(::std::sync::Arc::new(UuidV7Generator::builder().build()))
            .uuid_formatter(::std::sync::Arc::new(LowerUrnUuidFormatter::builder().build()))
            .task_repository(::std::sync::Arc::new(InMemoryTaskRepository::builder().build()))
            .build()
    }

    #[builder(finish_fn = build)]
    fn new_from_gateways(
        uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
        uuid_formatter: ::std::sync::Arc<dyn UuidFormatter + ::core::marker::Send + ::core::marker::Sync>,
        task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,
    ) -> Self {
        Self::builder()
            .create_task_boundary(::std::sync::Arc::new(CreateTaskInteractor::builder()
                .uuid_generator(::std::sync::Arc::clone(&uuid_generator))
                .task_repository(::std::sync::Arc::clone(&task_repository))
                .build()))
            .view_tasks_boundary(::std::sync::Arc::new(ViewTasksInteractor::builder()
                .uuid_generator(::std::sync::Arc::clone(&uuid_generator))
                .uuid_formatter(::std::sync::Arc::clone(&uuid_formatter))
                .task_repository(::std::sync::Arc::clone(&task_repository))
                .build()))
            .build()
    }

    #[wasm_bindgen(js_name = createTask)]
    pub fn create_task(&self, request: JsValue) -> JsValue {
        JsValue::null()
    }

    #[wasm_bindgen(js_name = viewTasks)]
    pub fn view_tasks(&self, request: JsValue) -> JsValue {
        JsValue::null()
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct CreateTaskRequest {
    pub task_description: String,
}

pub type CreateTaskResponse = ::core::result::Result<CreateTaskOkResponse, CreateTaskErrResponse>;

pub type CreateTaskOkResponse = ();

#[wasm_bindgen(getter_with_clone)]
pub enum CreateTaskErrResponse {
    // TaskDescriptionLengthUnderflow {
    //     actual: usize,
    //     min_expected: usize,
    // },
    // TaskDescriptionLengthOverflow {
    //     actual: usize,
    //     max_expected: usize,
    // },
    TaskDescriptionLengthUnderflow,
    TaskDescriptionLengthOverflow,
}

pub mod models {
    use ::use_cases::boundaries::models as boundaries;
    use ::wasm_bindgen::prelude::*;

    #[wasm_bindgen(getter_with_clone)]
    #[derive(::bon::Builder)]
    pub struct Task {
        pub id: String,
        pub description: String,
        pub status: TaskStatus,
        pub created_at: String,
    }

    impl ::core::convert::From<boundaries::Task> for Task {
        fn from(task: boundaries::Task) -> Self {
            Self::builder()
                .id(task.id.to_string())
                .description(task.description.to_string())
                .status(task.status.into())
                .created_at(task.created_at.to_string())
                .build()
        }
    }

    #[wasm_bindgen]
    #[derive(::core::clone::Clone, ::core::marker::Copy)]
    pub enum TaskStatus {
        Pending = "pending",
        InProgress = "in-progress",
        Completed = "completed",
    }

    impl ::core::convert::From<boundaries::TaskStatus> for TaskStatus {
        fn from(status: boundaries::TaskStatus) -> Self {
            match status {
                boundaries::TaskStatus::Pending => TaskStatus::Pending,
                boundaries::TaskStatus::InProgress => TaskStatus::InProgress,
                boundaries::TaskStatus::Completed => TaskStatus::Completed,
            }
        }
    }
}

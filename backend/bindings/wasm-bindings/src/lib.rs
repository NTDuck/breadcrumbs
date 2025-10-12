mod utils;

use ::use_cases::gateways::*;
use ::use_cases::boundaries::*;
use ::use_cases::interactors::*;
use ::infrastructures::*;
use ::wasm_bindgen::prelude::*;

use crate::utils::FallibleExt;
use crate::utils::JsFallible;

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
    pub async fn create_task(&self, request: CreateTaskRequest) -> JsFallible<CreateTaskResponse> {
        ::std::sync::Arc::clone(&self.create_task_boundary).apply(request).await
            .into_js()
    }

    #[wasm_bindgen(js_name = viewTasks)]
    pub async fn view_tasks(&self, request: ViewTasksRequest) -> JsFallible<ViewTasksResponse> {
        ::std::sync::Arc::clone(&self.view_tasks_boundary).apply(request).await
            .into_js()
    }
}

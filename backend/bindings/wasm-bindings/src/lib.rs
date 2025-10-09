use ::use_cases::gateways::*;
use ::use_cases::boundaries::*;
use ::use_cases::interactors::*;
use ::infrastructures::*;
use ::wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn addTwoNumbers(lhs: u32, rhs: u32) -> u32 {
    lhs + rhs
}

#[derive(::bon::Builder)]
#[builder(builder_type(vis = "pub(self)"))]
pub struct Application {
    create_task_boundary: ::std::sync::Arc<dyn CreateTaskBoundary>,
    view_tasks_boundary: ::std::sync::Arc<dyn ViewTasksBoundary>,
}

#[::bon::bon]
impl Application {
    pub fn new() -> Self {
        todo!()
    }

    #[builder]
    fn __new(
        uuid_generator: ::std::sync::Arc<dyn UuidGenerator>,
        uuid_formatter: ::std::sync::Arc<dyn UuidFormatter>,
        task_repository: ::std::sync::Arc<dyn TaskRepository>,
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
}

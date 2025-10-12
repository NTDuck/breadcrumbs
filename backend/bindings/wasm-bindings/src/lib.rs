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
    remove_task_boundary: ::std::sync::Arc<dyn RemoveTaskBoundary + ::core::marker::Send + ::core::marker::Sync>,
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
            .uuid_codec(::std::sync::Arc::new(LowerUrnUuidCodec::builder().build()))
            .task_repository(::std::sync::Arc::new(InMemoryTaskRepository::builder().build()))
            .build()
    }

    #[builder(finish_fn = build)]
    fn new_from_gateways(
        uuid_generator: ::std::sync::Arc<dyn UuidGenerator + ::core::marker::Send + ::core::marker::Sync>,
        uuid_codec: ::std::sync::Arc<dyn UuidCodec + ::core::marker::Send + ::core::marker::Sync>,
        task_repository: ::std::sync::Arc<dyn TaskRepository + ::core::marker::Send + ::core::marker::Sync>,
    ) -> Self {
        Self::builder()
            .create_task_boundary(::std::sync::Arc::new(CreateTaskInteractor::builder()
                .uuid_generator(::std::sync::Arc::clone(&uuid_generator))
                .task_repository(::std::sync::Arc::clone(&task_repository))
                .build()))
            .remove_task_boundary(::std::sync::Arc::new(RemoveTaskInteractor::builder()
                .uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                .task_repository(::std::sync::Arc::clone(&task_repository))
                .build()))
            .view_tasks_boundary(::std::sync::Arc::new(ViewTasksInteractor::builder()
                .uuid_generator(::std::sync::Arc::clone(&uuid_generator))
                .uuid_codec(::std::sync::Arc::clone(&uuid_codec))
                .task_repository(::std::sync::Arc::clone(&task_repository))
                .build()))
            .build()
    }

    #[wasm_bindgen(js_name = createTask)]
    pub async fn create_task(&self, request: CreateTaskRequest) -> JsPromise<CreateTaskOkResponse> {
        NestedFallibleExt::into_js(::std::sync::Arc::clone(&self.create_task_boundary).apply(request).await)
    }
    
    #[wasm_bindgen(js_name = removeTask)]
    pub async fn remove_task(&self, request: RemoveTaskRequest) -> JsPromise<RemoveTaskOkResponse> {
        NestedFallibleExt::into_js(::std::sync::Arc::clone(&self.remove_task_boundary).apply(request).await)
    }

    #[wasm_bindgen(js_name = viewTasks)]
    pub async fn view_tasks(&self, request: ViewTasksRequest) -> JsPromise<ViewTasksResponse> {
        FallibleExt::into_js(::std::sync::Arc::clone(&self.view_tasks_boundary).apply(request).await)
    }
}

pub type JsPromise<T = ()> = ::core::result::Result<T, ::wasm_bindgen::JsValue>;

pub(crate) trait FallibleExt<T = ()> {
    fn into_js(self) -> JsPromise<T>;
}

impl<T> FallibleExt<T> for ::aliases::result::Fallible<T> {
    fn into_js(self) -> JsPromise<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
    }
}

pub(crate) trait NestedFallibleExt<T = ()> {
    fn into_js(self) -> JsPromise<T>;
}

impl<T, E> NestedFallibleExt<T> for ::aliases::result::Fallible<::core::result::Result<T, E>>
where
    E: ::core::error::Error,
{
    fn into_js(self) -> JsPromise<T> {
        self.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string()))
            .and_then(|error| error.map_err(|error| ::wasm_bindgen::JsValue::from_str(&error.to_string())))
    }
}

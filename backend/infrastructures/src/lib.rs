use ::async_trait::async_trait;
use ::use_cases::gateways::*;
use ::use_cases::utils::pagination::PaginationRequest;
use ::use_cases::utils::pagination::PaginationResponse;
use ::use_cases::utils::pagination::PaginationRange;

pub struct UuidV7Generator;

#[::bon::bon]
impl UuidV7Generator {
    #[builder(builder_type(vis = "pub"))]
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UuidGenerator for UuidV7Generator {
    async fn generate(self: ::std::sync::Arc<Self>) -> ::aliases::result::Fallible<::domain::Uuid> {
        let uuid = ::uuid::Uuid::now_v7();

        let uuid = ::domain::Uuid::builder()
            .value(uuid.into_bytes())
            .build();

        ::aliases::result::Fallible::Ok(uuid)
    }

    async fn get_timestamp(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> ::aliases::result::Fallible<::aliases::time::Timestamp> {
        let uuid = ::uuid::Uuid::from_bytes(**uuid);

        match uuid.get_timestamp() {
            ::core::option::Option::Some(timestamp) => {
                let (seconds, nanoseconds) = timestamp.to_unix();
                let timestamp = ::chrono::DateTime::from_timestamp(seconds as i64, nanoseconds);

                match timestamp {
                    ::core::option::Option::Some(timestamp) => ::aliases::result::Fallible::Ok(timestamp.naive_local()),
                    ::core::option::Option::None => ::aliases::result::Fallible::Err(::anyhow::anyhow!("Out-of-range number of seconds and/or invalid nanosecond")),
                }
            }
            ::core::option::Option::None => ::aliases::result::Fallible::Err(::anyhow::anyhow!("Incorrect UUID version (must be v1, v6, or v7)")),
        }
    }
}

pub struct LowerUrnUuidFormatter;

#[::bon::bon]
impl LowerUrnUuidFormatter {
    #[builder(builder_type(vis = "pub"))]
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UuidFormatter for LowerUrnUuidFormatter {
    async fn format(self: ::std::sync::Arc<Self>, uuid: &::domain::Uuid) -> ::aliases::result::Fallible<::aliases::string::String> {
        let uuid = ::uuid::Uuid::from_bytes(**uuid);

        let mut buffer = [0u8; 45];
        let formatted = uuid.as_urn().encode_lower(&mut buffer);

        ::aliases::result::Fallible::Ok(formatted.to_string().into())
    }
}

#[derive(::bon::Builder)]
pub struct InMemoryTaskRepository {
    // Stores UUID in reverse chronological order (v7 is time-ordered)
    // See: https://www.rfc-editor.org/rfc/rfc9562.html#name-uuid-version-7
    #[builder(skip)]
    tasks_by_ids: ::tokio::sync::Mutex<::std::collections::BTreeMap<::core::cmp::Reverse<::domain::Uuid>, ::domain::Task>>,
}

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn save(self: ::std::sync::Arc<Self>, task: ::domain::Task) -> ::aliases::result::Fallible {
        self.tasks_by_ids.lock().await.insert(::core::cmp::Reverse(task.id), task);

        ::aliases::result::Fallible::Ok(())
    }

    async fn remove(self: ::std::sync::Arc<Self>, task_id: ::domain::Uuid) -> ::aliases::result::Fallible {
        self.tasks_by_ids.lock().await.remove(&::core::cmp::Reverse(task_id));

        ::aliases::result::Fallible::Ok(())
    }

    async fn show(self: ::std::sync::Arc<Self>, pagination_request: PaginationRequest) -> ::aliases::result::Fallible<PaginationResponse<::domain::Task>> {
        let pagination_range = PaginationRange::from(pagination_request);

        let tasks = self.tasks_by_ids.lock().await;

        let total = tasks.len();
        let tasks = tasks.iter()
            .skip(pagination_range.offset)
            .take(pagination_range.limit)
            .map(|(_, task)| task)
            .cloned()
            .collect::<::std::vec::Vec<_>>();

        let tasks = PaginationResponse::builder()
            .items(tasks)
            .request(pagination_request)
            .total(total)
            .build();

        ::aliases::result::Fallible::Ok(tasks)
    }
}

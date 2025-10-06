use ::async_trait::async_trait;
use ::domain::Uuid;

// Possible impls: v1 (/snowflake?), v4, v7, and perhaps more :3
#[async_trait]
pub trait UuidGenerator {
    async fn generate(self: ::std::sync::Arc<Self>) -> Uuid;
}

#[derive(::bon::Builder)]
pub struct Task {
    pub id: TaskId,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

pub type TaskId = Snowflake;

#[derive(::bon::Builder)]
pub struct TaskDescription {
    value: ::std::borrow::Cow<'static, str>,
}

#[derive(::core::default::Default)]
pub enum TaskStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
}

pub struct Snowflake {
    // also guide on this, builder(with ...)
}

// use chronos instead
pub type Timestamp = ::std::time::SystemTime;
pub type Interval = ::std::time::Duration;

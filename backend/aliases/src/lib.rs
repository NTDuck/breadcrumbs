pub mod result {
    pub type Fallible<T = ()> = ::core::result::Result<T, ::anyhow::Error>;
}

pub mod time {
    pub type Timestamp = ::chrono::NaiveDateTime;
    pub type Interval = ::chrono::Duration;
}

pub mod string {
    pub type String = ::std::borrow::Cow<'static, str>;
}

#[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::Eq, ::core::cmp::PartialEq, ::core::cmp::Ord, ::core::cmp::PartialOrd)]
pub struct Uuid([u8; 16]);

#[::bon::bon]
impl Uuid {
    #[builder]
    pub fn new(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl ::core::ops::Deref for Uuid {
    type Target = [u8; 16];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(::core::clone::Clone)]
#[derive(::bon::Builder)]
pub struct Task {
    pub id: Uuid,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

#[derive(::core::clone::Clone)]
pub struct TaskDescription(::aliases::string::String);

#[::bon::bon]
impl TaskDescription {
    #[builder(on(::aliases::string::String, into))]
    pub fn new(value: ::aliases::string::String) -> ::core::result::Result<Self, TaskDescriptionError> {
        let value = Self::normalize(value);
        Self::validate(value).map(Self)
    }

    fn normalize(value: ::aliases::string::String) -> ::aliases::string::String {
        match value {
            ::std::borrow::Cow::Borrowed(value) => {
                let trimmed = value.trim();

                if trimmed.len() == value.len() {
                    ::std::borrow::Cow::Borrowed(trimmed)
                } else {
                    ::std::borrow::Cow::Owned(trimmed.to_string())
                }
            },

            ::std::borrow::Cow::Owned(value) => {
                let trimmed = value.trim();

                ::std::borrow::Cow::Owned(trimmed.to_string())
            },
        }
    }

    fn validate(value: ::aliases::string::String) -> ::core::result::Result<::aliases::string::String, TaskDescriptionError> {
        if value.len() < Self::MIN_EXPECTED_LENGTH {
            let error = TaskDescriptionError::underflow()
                .length(value.len())
                .build();

            ::core::result::Result::Err(error)

        } else if value.len() > Self::MAX_EXPECTED_LENGTH {
            let error = TaskDescriptionError::overflow()
                .length(value.len())
                .build();

            ::core::result::Result::Err(error)

        } else {
            ::core::result::Result::Ok(value)
        }
    }

    const MIN_EXPECTED_LENGTH: usize = 1;
    const MAX_EXPECTED_LENGTH: usize = 1024;
}

impl ::core::ops::Deref for TaskDescription {
    type Target = ::aliases::string::String;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum TaskDescriptionError {
    LengthUnderflow {
        actual: usize,
        min_expected: usize,
    },
    LengthOverflow {
        actual: usize,
        max_expected: usize,
    },
}

#[::bon::bon]
impl TaskDescriptionError {
    #[builder(finish_fn = build)]
    pub fn underflow(length: usize) -> Self {
        Self::LengthUnderflow {
            actual: length,
            min_expected: TaskDescription::MIN_EXPECTED_LENGTH,
        }
    }

    #[builder(finish_fn = build)]
    pub fn overflow(length: usize) -> Self {
        Self::LengthOverflow {
            actual: length,
            max_expected: TaskDescription::MAX_EXPECTED_LENGTH,
        }
    }
}

#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

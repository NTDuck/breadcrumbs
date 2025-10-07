use crate::utils::aliases;

mod utils;

// Note that derived traits will be added when needed
// Do not repeat axiom (it sucks)

// Implicit requirement is that Uuid 
#[derive(::bon::Builder)]
#[builder(const)]
pub struct Uuid {
    #[allow(dead_code)]
    value: [u8; 16],
}

#[derive(::bon::Builder)]
pub struct Task {
    pub id: Uuid,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

pub struct TaskDescription {
    value: aliases::string::String,
}

#[::bon::bon]
impl TaskDescription {
    #[builder(builder_type(vis = "pub"), on(_, into))]
    fn new(value: aliases::string::String) -> ::core::result::Result<Self, TaskDescriptionError> {
        let value = Self::normalize(value);
        Self::validate(value).map(|value| Self { value })
    }

    fn normalize(value: aliases::string::String) -> aliases::string::String {
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

    fn validate(value: aliases::string::String) -> ::core::result::Result<aliases::string::String, TaskDescriptionError> {
        if value.len() < Self::MIN_EXPECTED_LENGTH {
            ::core::result::Result::Err(TaskDescriptionError::LengthUnderflow {
                actual: value.len() as usize,
                min_expected: Self::MIN_EXPECTED_LENGTH,
            })
        } else if value.len() > Self::MAX_EXPECTED_LENGTH {
            ::core::result::Result::Err(TaskDescriptionError::LengthOverflow {
                actual: value.len() as usize,
                max_expected: Self::MAX_EXPECTED_LENGTH,
            })
        } else {
            ::core::result::Result::Ok(value)
        }
    }

    const MIN_EXPECTED_LENGTH: usize = 1;
    const MAX_EXPECTED_LENGTH: usize = 1024;
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

impl ::core::ops::Deref for TaskDescription {
    type Target = aliases::string::String;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

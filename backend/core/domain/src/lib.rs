// Note that derived traits will be added when needed
// Do not repeat axiom (it sucks)

// Implicit requirement is that Uuid 
#[derive(::bon::Builder)]
pub struct Uuid {
    value: [u8; 16],
}

#[derive(::bon::Builder)]
pub struct Task {
    pub id: Uuid,
    pub description: TaskDescription,
    pub status: TaskStatus,
}

pub struct TaskDescription {
    value: ::std::borrow::Cow<'static, str>,
}

#[::bon::bon]
impl TaskDescription {
    #[builder(builder_type(vis = "pub"), on(_, into))]
    fn new(value: ::std::borrow::Cow<'static, str>) -> ::core::result::Result<Self, TaskDescriptionError> {
        // let value = value.trim();

        if (value.len() as u64) < Self::MIN_EXPECTED_LENGTH {
            ::core::result::Result::Err(TaskDescriptionError::LengthUnderflow {
                actual: value.len() as u64,
                min_expected: Self::MIN_EXPECTED_LENGTH,
            })
        } else if (value.len() as u64) > Self::MAX_EXPECTED_LENGTH {
            ::core::result::Result::Err(TaskDescriptionError::LengthOverflow {
                actual: value.len() as u64,
                max_expected: Self::MAX_EXPECTED_LENGTH,
            })
        } else {
            ::core::result::Result::Ok(TaskDescription { value })
        }
    }

    const MIN_EXPECTED_LENGTH: u64 = 1;
    const MAX_EXPECTED_LENGTH: u64 = 1024;
}

pub enum TaskDescriptionError {
    LengthUnderflow {
        actual: u64,
        min_expected: u64,
    },
    LengthOverflow {
        actual: u64,
        max_expected: u64,
    },
}

impl ::core::ops::Deref for TaskDescription {
    type Target = str;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

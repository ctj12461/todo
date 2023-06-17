use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Priority(i32);

impl Priority {
    pub const MIN: i32 = -3;
    pub const MAX: i32 = 3;

    pub fn upgrade(&mut self) {
        self.0 = i32::min(self.0 + 1, Priority::MAX);
    }

    pub fn downgrade(&mut self) {
        self.0 = i32::max(self.0 - 1, Priority::MIN);
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl From<i32> for Priority {
    fn from(value: i32) -> Self {
        Priority(value.clamp(Priority::MIN, Priority::MAX))
    }
}

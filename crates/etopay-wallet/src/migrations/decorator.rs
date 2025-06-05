use serde::{Deserialize, Serialize};

use crate::MigrationStatus;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithMigrationStatus<T> {
    #[serde(skip)]
    pub migration_status: MigrationStatus,

    #[serde(flatten)]
    pub data: T,
}

impl<T> WithMigrationStatus<T> {
    pub fn new(data: T, status: MigrationStatus) -> Self {
        Self {
            data,
            migration_status: status,
        }
    }

    pub fn mark_completed(mut self) -> Self {
        self.migration_status = MigrationStatus::Completed;
        self
    }

    pub fn inner(&self) -> &T {
        &self.data
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}

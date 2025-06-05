use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MigrationStatus {
    // indicates that after migrating to the new version, some fields need to be filled in manually
    Pending,
    // indicates that the object is fully populated and the migration process is complete
    Completed,
}

impl Default for MigrationStatus {
    fn default() -> Self {
        MigrationStatus::Completed
    }
}

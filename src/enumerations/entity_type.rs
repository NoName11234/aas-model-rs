use serde::{Deserialize, Serialize};

///Enumeration for denoting whether an entity is a self-managed entity or a co-managed entity.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum EntityType {
    ///There is no separate Asset Administration Shell for co-managed entities. Co-managed entities
    /// need to be part of a self-managed entity.
    CoManagedEntity,
    ///Self-managed entities have their own Asset Administration Shell but can be part of another
    /// composite self-managed entity.
    SelfManagedEntity
}
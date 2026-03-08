use serde::{Deserialize, Serialize};

use super::action::ActionTag;
use super::cohort::MetricField;
use super::ids::{CapabilityId, ConceptId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryMechanism {
    /// Agents discover it by observing the physical world.
    Observation,
    /// Probabilistic discovery through repeated relevant action.
    TrialAndError,
    /// Received from another civilization.
    Transmission,
    /// Player-granted; bypasses prerequisites.
    DivineGnosis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: CapabilityId,
    /// e.g., "fire_starting", "agriculture", "iron_smelting", "writing"
    pub label: String,
    pub prerequisite_capabilities: Vec<CapabilityId>,
    pub prerequisite_concepts: Vec<ConceptId>,
    pub metric_thresholds: Vec<(MetricField, f32)>,
    /// Actions that become available once this capability exists in the world.
    pub unlocked_actions: Vec<ActionTag>,
    /// Concepts that can now emerge once this capability exists.
    pub unlocks_concepts: Vec<ConceptId>,
    pub discovery_mechanism: DiscoveryMechanism,
}

use serde::{Deserialize, Serialize};

/// Classifies actions for utility modification and capability gating.
///
/// Referenced by `UtilityModifier.action_tag` and `Capability.unlocked_actions`.
/// This list is open-ended — avoid building systems that assume a fixed count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ActionTag {
    // Subsistence
    Forage,
    Hunt,
    Fish,
    Farm,

    // Resource management
    ShareResources,
    Redistribute,
    HoardResources,
    Trade,

    // Conflict
    Raid,
    Defend,
    Flee,

    // Leadership and social
    DeferToHierarchy,
    AssertDominance,
    Negotiate,
    Socialize,

    // Cultural and epistemic
    TeachConcept,
    PerformRitual,
    Explore,
    Innovate,

    // Self-care
    Rest,
}

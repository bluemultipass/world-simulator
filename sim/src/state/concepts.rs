use serde::{Deserialize, Serialize};

use super::action::ActionTag;
use super::cohort::MetricField;
use super::ids::ConceptId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConceptType {
    /// A named supernatural agent; may accumulate theology.
    Deity,
    /// Political/social/economic framework; shapes collective behavior.
    Ideology,
    /// A claim about how the world works; right or wrong.
    CausalModel,
    /// A behavioral prohibition; follows rule-without-understanding dynamics.
    Taboo,
    /// A formal role structure: priesthood, law, market, army.
    Institution,
    /// An agentive interpretation of a natural thing: thunder-being, river spirit.
    NaturalPhenomenon,
}

/// Defines how holding a concept at a given strength modifies action utility scores.
/// Applied during action selection for agents whose belief strength exceeds the threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityModifier {
    /// Which class of actions this affects.
    pub action_tag: ActionTag,
    /// Positive = bonus, negative = penalty; magnitude scales with belief strength.
    pub direction: f32,
    /// Minimum belief strength for this modifier to apply.
    pub threshold: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TransmissionMedium {
    /// Spoken transmission; lossy, mutates with distance and time.
    #[default]
    Oral,
    /// Text-based; stable but requires literacy capability.
    Written,
    /// Embodied practice; stable form, but meaning drifts without Written backup.
    Ritual,
    /// First-hand witness; highest initial strength, not directly re-transmissible.
    DirectObservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmissionProfile {
    /// Probability of transmission per social contact per year.
    pub base_rate: f32,
    /// Probability that a transmitted copy drifts from the original.
    pub mutation_rate: f32,
    pub required_medium: TransmissionMedium,
    /// Whether high-charisma transmitters dramatically increase spread rate.
    pub charisma_amplified: bool,
}

impl Default for TransmissionProfile {
    fn default() -> Self {
        Self {
            base_rate: 0.0,
            mutation_rate: 0.0,
            required_medium: TransmissionMedium::default(),
            charisma_amplified: false,
        }
    }
}

/// What must be true in the local civilization for this concept to come into existence.
/// Always checked against the specific civilization's state — not global.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergenceConditions {
    pub metric_thresholds: Vec<(MetricField, f32)>,
    pub required_concepts: Vec<ConceptId>,
    pub population_minimum: Option<u32>,
    /// If true, can only enter via player divine action.
    pub player_intervention: bool,
}

impl Default for EmergenceConditions {
    fn default() -> Self {
        Self {
            metric_thresholds: Vec::new(),
            required_concepts: Vec::new(),
            population_minimum: None,
            player_intervention: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: ConceptId,
    /// Human-readable name, e.g., "Asha", "communism", "germ theory".
    pub label: String,
    pub concept_type: ConceptType,
    /// Behavioral effects on agents who hold this belief.
    pub utility_modifiers: Vec<UtilityModifier>,
    pub transmission: TransmissionProfile,
    pub emergence_conditions: EmergenceConditions,
    /// Concepts this one competes with.
    pub conflicts_with: Vec<ConceptId>,
}

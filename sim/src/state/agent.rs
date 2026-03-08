use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::ids::{AgentId, CohortId, ConceptId, Domain, TileId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Sex {
    #[default]
    Male,
    Female,
}

/// Per-need satisfaction levels. All `f32` in `[0.0, 1.0]`.
/// `0.0` = completely unmet / critical, `1.0` = fully satisfied.
/// Decay exponentially over time; agents act to restore them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNeeds {
    pub food: f32,
    pub water: f32,
    pub sleep: f32,
    pub shelter: f32,
    pub warmth: f32,
    pub safety: f32,
    pub belonging: f32,
    pub status: f32,
    pub meaning: f32,
}

impl Default for AgentNeeds {
    fn default() -> Self {
        Self {
            food: 0.0,
            water: 0.0,
            sleep: 0.0,
            shelter: 0.0,
            warmth: 0.0,
            safety: 0.0,
            belonging: 0.0,
            status: 0.0,
            meaning: 0.0,
        }
    }
}

/// Stable dispositional values. Inherited with mutation at birth.
/// Influence utility weight curves, not hard behavior rules.
/// All `f32` in `[0.0, 1.0]`. This list will grow; avoid fixed-count assumptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTraits {
    pub brave: f32,
    pub cautious: f32,
    pub aggressive: f32,
    pub empathetic: f32,
    pub curious: f32,
    pub credulous: f32,
    pub charismatic: f32,
    pub dominant: f32,
    pub deferential: f32,
    pub compassionate: f32,
    pub tribal: f32,
}

impl Default for AgentTraits {
    fn default() -> Self {
        Self {
            brave: 0.0,
            cautious: 0.0,
            aggressive: 0.0,
            empathetic: 0.0,
            curious: 0.0,
            credulous: 0.0,
            charismatic: 0.0,
            dominant: 0.0,
            deferential: 0.0,
            compassionate: 0.0,
            tribal: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BondType {
    #[default]
    Kin,
    Friendship,
    Romantic,
    Hierarchy,
    Institutional,
}

/// Family relationship from the perspective of the agent holding the record.
/// Present only when `bond_type` is `Kin`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KinRelation {
    Parent,
    Child,
    Sibling,
    Grandparent,
    Grandchild,
    Cousin,
    AuntOrUncle,
    NieceOrNephew,
    Spouse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub trust: f32,
    pub affection: f32,
    pub rivalry: f32,
    pub bond_type: BondType,
    pub kin_relation: Option<KinRelation>,
    /// For attenuation — relationships decay without contact.
    pub last_interaction_tick: u64,
}

impl Default for Relationship {
    fn default() -> Self {
        Self {
            trust: 0.0,
            affection: 0.0,
            rivalry: 0.0,
            bond_type: BondType::default(),
            kin_relation: None,
            last_interaction_tick: 0,
        }
    }
}

/// Opaque reference into the global event log.
/// The referenced event is immutable once archived.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventRef {
    pub tick: u64,
    pub event_id: u64,
}

/// How accurately an agent or cohort understands a domain or concept.
/// Ordered from least to most accurate; derive ordering uses declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum KnowledgeState {
    #[default]
    Ignorance,
    Misattribution,
    RuleWithoutUnderstanding,
    PartialUnderstanding,
    FullUnderstanding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefEntry {
    pub concept_id: ConceptId,
    pub strength: f32,
    pub generation_distance: u32,
    pub knowledge_state: KnowledgeState,
}

impl Default for BeliefEntry {
    fn default() -> Self {
        Self {
            concept_id: ConceptId::default(),
            strength: 0.0,
            generation_distance: 0,
            knowledge_state: KnowledgeState::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub tick: u64,
    pub event: EventRef,
    pub salience: f32,
    pub interpretation: String,
}

impl Default for MemoryEntry {
    fn default() -> Self {
        Self {
            tick: 0,
            event: EventRef::default(),
            salience: 0.0,
            interpretation: String::new(),
        }
    }
}

/// Full-fidelity named agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub name: String,
    pub age: f32,
    pub sex: Sex,
    pub location: TileId,
    pub cohort_id: CohortId,
    pub needs: AgentNeeds,
    pub traits: AgentTraits,
    /// Sparse — only agents this agent has a relationship with.
    pub relationships: BTreeMap<AgentId, Relationship>,
    /// Ordered by recency. Pruned by emotional salience over time.
    pub personal_memory: Vec<MemoryEntry>,
    /// What the agent believes about the world.
    pub cultural_memory: BTreeMap<ConceptId, BeliefEntry>,
    /// Per-capability mastery. `Domain` is an alias for `CapabilityId`.
    pub knowledge: BTreeMap<Domain, KnowledgeState>,
}

impl Default for Agent {
    fn default() -> Self {
        Self {
            id: AgentId::default(),
            name: String::new(),
            age: 0.0,
            sex: Sex::default(),
            location: TileId::default(),
            cohort_id: CohortId::default(),
            needs: AgentNeeds::default(),
            traits: AgentTraits::default(),
            relationships: BTreeMap::new(),
            personal_memory: Vec::new(),
            cultural_memory: BTreeMap::new(),
            knowledge: BTreeMap::new(),
        }
    }
}

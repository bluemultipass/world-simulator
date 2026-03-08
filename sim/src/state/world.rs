use std::collections::BTreeMap;

use rand_chacha::ChaChaRng;
use serde::{Deserialize, Serialize};

use super::agent::Agent;
use super::capabilities::Capability;
use super::civ::Civilization;
use super::cohort::Cohort;
use super::concepts::Concept;
use super::ids::{AgentId, CapabilityId, CivId, CohortId, ConceptId};
use super::physical::PhysicalWorld;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldClock {
    /// Negative = BCE. Starts around -300,000.
    pub year: i64,
    /// Monotonically increasing tick counter.
    pub tick: u64,
    /// Years elapsed in the most recent tick.
    pub last_delta: f32,
}

impl Default for WorldClock {
    fn default() -> Self {
        Self {
            year: -300_000,
            tick: 0,
            last_delta: 0.0,
        }
    }
}

/// Immutable store for dead agents. Written once at death; never modified after.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentArchive {
    /// Keyed by id. Entry is sealed at the death tick.
    pub agents: BTreeMap<AgentId, Agent>,
}

impl Default for AgentArchive {
    fn default() -> Self {
        Self {
            agents: BTreeMap::new(),
        }
    }
}

/// The top-level simulation container. Serialized in full for save/load and determinism replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    /// Original PRNG seed. Serialized with state.
    pub seed: u64,
    /// Current PRNG state. Deterministic.
    pub rng: ChaChaRng,
    pub clock: WorldClock,
    /// Active named agents with individual tracking.
    pub agents: BTreeMap<AgentId, Agent>,
    /// All cohorts at variable fidelity.
    pub cohorts: BTreeMap<CohortId, Cohort>,
    /// All civilizations including the focus civ.
    pub civilizations: BTreeMap<CivId, Civilization>,
    /// Which civilization is the player's.
    pub focus_civ_id: CivId,
    pub world: PhysicalWorld,
    /// Immutable. Dead agents only. Nothing writes here after death.
    pub archive: AgentArchive,
    /// Definition registry for all concepts that exist anywhere in the simulation.
    pub concepts: BTreeMap<ConceptId, Concept>,
    /// Definition registry for all capabilities that have been discovered anywhere.
    pub capabilities: BTreeMap<CapabilityId, Capability>,
}

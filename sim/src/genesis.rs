// World generation from a u64 seed via ChaChaRng.

use std::collections::BTreeMap;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::state::agent::{Agent, AgentNeeds, AgentTraits, Sex};
use crate::state::civ::Civilization;
use crate::state::cohort::{Cohort, PopulationState};
use crate::state::ids::{AgentId, CivId, CohortId, RegionId, TileId};
use crate::state::physical::{PhysicalWorld, RegenerationRates, ResourceLevels, TerrainType, Tile};
use crate::state::world::{AgentArchive, WorldClock, WorldState};

const AGENT_COUNT: u32 = 8;

const AGENT_NAMES: &[&str] = &[
    "Akar", "Bela", "Coru", "Danu", "Erra", "Fira", "Gorn", "Halu",
];

/// Build all 25 tiles for a 5×5 grid. IDs are 1–25 in row-major order.
///
/// Layout (0-indexed row, col):
/// - Central 3×3 (rows 1–3, cols 1–3): Grassland — high food and wood.
/// - Border ring: alternates Mountain / Forest by (row + col) parity.
fn build_tiles() -> BTreeMap<TileId, Tile> {
    let mut tiles = BTreeMap::new();
    for row in 0..5u64 {
        for col in 0..5u64 {
            let id = TileId(row * 5 + col + 1);
            let is_central = (1..=3).contains(&row) && (1..=3).contains(&col);

            let (terrain, resources, regen, elevation, carrying_capacity) = if is_central {
                (
                    TerrainType::Grassland,
                    ResourceLevels {
                        food: 1.0,
                        water: 0.8,
                        stone: 0.1,
                        wood: 0.8,
                        metal: 0.0,
                    },
                    RegenerationRates {
                        food: 0.3,
                        water: 0.5,
                        stone: 0.01,
                        wood: 0.2,
                        metal: 0.0,
                    },
                    50.0_f32,
                    20_u32,
                )
            } else if (row + col) % 2 == 0 {
                (
                    TerrainType::Mountain,
                    ResourceLevels {
                        food: 0.1,
                        water: 0.3,
                        stone: 1.0,
                        wood: 0.1,
                        metal: 0.3,
                    },
                    RegenerationRates {
                        food: 0.02,
                        water: 0.1,
                        stone: 0.05,
                        wood: 0.01,
                        metal: 0.01,
                    },
                    1500.0_f32,
                    3_u32,
                )
            } else {
                (
                    TerrainType::Forest,
                    ResourceLevels {
                        food: 0.5,
                        water: 0.6,
                        stone: 0.1,
                        wood: 1.0,
                        metal: 0.0,
                    },
                    RegenerationRates {
                        food: 0.1,
                        water: 0.3,
                        stone: 0.01,
                        wood: 0.3,
                        metal: 0.0,
                    },
                    200.0_f32,
                    10_u32,
                )
            };

            tiles.insert(
                id,
                Tile {
                    id,
                    terrain,
                    elevation,
                    resources,
                    resource_regeneration: regen,
                    carrying_capacity,
                },
            );
        }
    }
    tiles
}

fn sample_traits(rng: &mut ChaChaRng) -> AgentTraits {
    AgentTraits {
        brave: rng.gen::<f32>(),
        cautious: rng.gen::<f32>(),
        aggressive: rng.gen::<f32>(),
        empathetic: rng.gen::<f32>(),
        curious: rng.gen::<f32>(),
        credulous: rng.gen::<f32>(),
        charismatic: rng.gen::<f32>(),
        dominant: rng.gen::<f32>(),
        deferential: rng.gen::<f32>(),
        compassionate: rng.gen::<f32>(),
        tribal: rng.gen::<f32>(),
    }
}

/// Produce a minimal but valid starting world deterministically from `seed`.
///
/// - 5×5 tile grid (TileId 1–25), central 3×3 Grassland, border ring Mountain/Forest.
/// - One focus civilization with one cohort and `AGENT_COUNT` named agents.
/// - All agents placed on the central tile (TileId 13).
/// - All needs initialised at 0.9; traits sampled uniformly from `[0, 1)`.
/// - Clock: year = −300 000, tick = 0, last_delta = 0.0.
pub fn genesis(seed: u64) -> WorldState {
    let mut rng = ChaChaRng::seed_from_u64(seed);

    let cohort_id = CohortId(1);
    let civ_id = CivId(1);
    // Central tile: row 2, col 2 (0-indexed) → id = 2*5 + 2 + 1 = 13.
    let starting_tile = TileId(13);

    let mut agents: BTreeMap<AgentId, Agent> = BTreeMap::new();
    for i in 0..AGENT_COUNT {
        let id = AgentId(u64::from(i) + 1);
        let sex = if rng.gen_bool(0.5) {
            Sex::Male
        } else {
            Sex::Female
        };
        let age: f32 = rng.gen_range(15.0_f32..=45.0_f32);
        let traits = sample_traits(&mut rng);

        let agent = Agent {
            id,
            name: AGENT_NAMES[i as usize].to_string(),
            age,
            sex,
            location: starting_tile,
            cohort_id,
            needs: AgentNeeds {
                food: 0.9,
                water: 0.9,
                sleep: 0.9,
                shelter: 0.9,
                warmth: 0.9,
                safety: 0.9,
                belonging: 0.9,
                status: 0.9,
                meaning: 0.9,
            },
            traits,
            ..Agent::default()
        };
        agents.insert(id, agent);
    }

    let cohort = Cohort {
        id: cohort_id,
        label: "Initial Band".to_string(),
        population: PopulationState {
            count: AGENT_COUNT,
            growth_rate: 0.0,
        },
        location: starting_tile,
        ..Cohort::default()
    };

    let civilization = Civilization {
        id: civ_id,
        label: "Focus Civilization".to_string(),
        cohort_id,
        location: RegionId(1),
        ..Civilization::default()
    };

    let mut cohorts = BTreeMap::new();
    cohorts.insert(cohort_id, cohort);

    let mut civilizations = BTreeMap::new();
    civilizations.insert(civ_id, civilization);

    WorldState {
        seed,
        rng,
        clock: WorldClock::default(),
        agents,
        cohorts,
        civilizations,
        focus_civ_id: civ_id,
        world: PhysicalWorld {
            tiles: build_tiles(),
            ..PhysicalWorld::default()
        },
        archive: AgentArchive::default(),
        concepts: BTreeMap::new(),
        capabilities: BTreeMap::new(),
    }
}

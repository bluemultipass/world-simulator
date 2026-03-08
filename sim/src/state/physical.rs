use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::ids::TileId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TerrainType {
    #[default]
    Grassland,
    Forest,
    Desert,
    Mountain,
    Wetland,
    Coast,
}

/// Current extractable resource quantities on a tile.
/// All `f32`, representing available units relative to a per-terrain baseline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceLevels {
    pub food: f32,
    pub water: f32,
    pub stone: f32,
    pub wood: f32,
    pub metal: f32,
}

/// Per-resource natural replenishment rate. Units per year.
/// Affected by climate and capability (e.g., farming raises effective food regeneration).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegenerationRates {
    pub food: f32,
    pub water: f32,
    pub stone: f32,
    pub wood: f32,
    pub metal: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tile {
    pub id: TileId,
    pub terrain: TerrainType,
    /// Meters.
    pub elevation: f32,
    pub resources: ResourceLevels,
    pub resource_regeneration: RegenerationRates,
    /// Max sustainable population given current resources and tech.
    pub carrying_capacity: u32,
}

/// Global climate parameters. Evolves each tick via slow drift and occasional shocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateState {
    /// Global offset from baseline; affects terrain productivity and warmth need decay.
    pub temperature: f32,
    /// Global moisture level; affects food and water availability.
    pub precipitation: f32,
    /// Rate of climate drift; high = faster change, more frequent shocks.
    pub volatility: f32,
}

impl Default for ClimateState {
    fn default() -> Self {
        Self {
            temperature: 0.0,
            precipitation: 0.5,
            volatility: 0.1,
        }
    }
}

/// A single active disease population and its spread state.
/// A world may have multiple active vectors simultaneously.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiseaseVector {
    pub label: String,
    /// Transmission probability per contact per year.
    pub virulence: f32,
    /// Death probability given infection.
    pub lethality: f32,
    /// Rate at which acquired immunity fades, per year.
    pub immunity_decay: f32,
    /// Tiles currently experiencing active spread.
    pub active_tiles: Vec<TileId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhysicalWorld {
    pub tiles: BTreeMap<TileId, Tile>,
    pub climate: ClimateState,
    pub disease_vectors: Vec<DiseaseVector>,
}

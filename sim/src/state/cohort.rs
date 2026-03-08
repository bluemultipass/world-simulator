use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::ids::{AgentId, CapabilityId, CohortId, ConceptId, TileId};

/// Current value and smoothed rate of change for a civilizational metric.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricValue {
    /// Current position `[0.0, 1.0]`.
    pub value: f32,
    /// Smoothed rate of change, per year; negative = declining.
    pub velocity: f32,
}

/// Identifies a specific metric for threshold comparisons (e.g., `EmergenceConditions`,
/// `Capability.metric_thresholds`). Covers both `CivilizationalMetrics` fields and
/// cohort-level summary fields available on all cohorts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MetricField {
    // CivilizationalMetrics fields
    SocialScale,
    AdministrativeComplexity,
    TerritorialCoherence,
    SpecializationIndex,
    SurplusCapacity,
    RitualSpecialization,
    LeadershipConcentration,
    RedistributionCentrality,

    // Cohort summary fields (available for all cohorts including sparse ones)
    Cohesion,
    ResourcePressure,
    CapabilityLevel,
}

/// Rough demographic breakdown. Full-pipeline only; `None` for sparse cohorts.
/// Fractions sum to 1.0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeDistribution {
    /// Fraction of population under ~15 years.
    pub children: f32,
    /// Fraction ~15–60 years.
    pub adults: f32,
    /// Fraction over ~60 years.
    pub elders: f32,
}

impl Default for AgeDistribution {
    fn default() -> Self {
        Self {
            children: 0.0,
            adults: 1.0,
            elders: 0.0,
        }
    }
}

/// Aggregate need satisfaction across the cohort. Mirrors per-need fields on
/// `Agent.needs`, averaged over all members. Full-pipeline only.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NeedSatisfactionRates {
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

/// Mean and variance per trait across the cohort. Full-pipeline only.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraitDistribution {
    /// Mean trait value across cohort members.
    pub means: BTreeMap<String, f32>,
    /// Variance per trait; high variance = diverse cohort.
    pub variances: BTreeMap<String, f32>,
}

/// Used where population is a raw count rather than a normalized value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PopulationState {
    /// Current headcount.
    pub count: u32,
    /// Smoothed fractional change per year; -0.03 = 3% annual decline.
    pub growth_rate: f32,
}

/// Represents the full population of a group — named agents included.
/// The same struct is used for all cohorts; pipeline depth varies by fidelity level.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cohort {
    pub id: CohortId,
    pub label: String,
    /// Total headcount including individually-tracked members. All cohorts.
    pub population: PopulationState,
    /// Internal unity. Falling → fragmentation events. All cohorts.
    pub cohesion: MetricValue,
    /// Food and land stress relative to population. All cohorts.
    pub resource_pressure: MetricValue,
    /// Rough proxy for military and economic capacity relative to neighbors. All cohorts.
    pub capability_level: MetricValue,
    /// Rough demographic breakdown. Full-pipeline only.
    pub age_distribution: Option<AgeDistribution>,
    /// Aggregate satisfaction rates per need. Full-pipeline only.
    pub need_satisfaction: Option<NeedSatisfactionRates>,
    /// Mean and variance per trait across cohort. Full-pipeline only.
    pub trait_distribution: Option<TraitDistribution>,
    /// Aggregate belief strength per concept.
    pub belief_profile: BTreeMap<ConceptId, f32>,
    /// Aggregate mastery level per capability.
    pub capability_profile: BTreeMap<CapabilityId, f32>,
    /// Centroid or primary tile.
    pub location: TileId,
    /// Named leader if one has emerged. Full-pipeline only.
    pub affiliation: Option<AgentId>,
}

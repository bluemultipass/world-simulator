use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::cohort::MetricValue;
use super::ids::{CivId, CohortId, RegionId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CivContactType {
    /// One civ attacks another for resources or territory.
    Raid,
    /// Resource exchange.
    Trade,
    /// Population movement across civ boundaries.
    Migration,
    /// Organized, sustained violence; larger scale than a raid.
    Conflict,
    /// Proximity-based concept or capability diffusion opportunity.
    CulturalContact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactOutcome {
    /// Initiator achieved their goal.
    Success,
    /// Initiator was repelled or objective unmet.
    Failure,
    /// Mixed result.
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivContactEntry {
    pub tick: u64,
    pub contact_type: CivContactType,
    pub initiator: CivId,
    pub outcome: ContactOutcome,
    /// Historical weight. High-salience entries decay slower and are pruned last.
    pub salience: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementType {
    /// Obligation to assist if the other is attacked.
    MutualDefense,
    /// Commitment not to raid or attack.
    NonAggression,
    /// One-way resource flow; encodes asymmetric power.
    Tribute,
    /// Formalized trade with mutual expectations.
    TradeCompact,
    /// Broad cooperation; typically implies mutual defense.
    Alliance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgreementStatus {
    Active,
    /// Who broke it and when; informs hostility and future trust.
    Broken {
        by: CivId,
        at_tick: u64,
    },
    /// Lapsed without violation.
    Expired,
}

/// Formal commitment between two civilizations.
/// Broken agreements are retained with `AgreementStatus::Broken` rather than deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivAgreement {
    pub agreement_type: AgreementType,
    pub formed_tick: u64,
    pub status: AgreementStatus,
}

/// Bilateral record between two civilizations with actual contact history.
/// Both sides maintain independent entries; they may diverge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CivRelation {
    pub first_contact_tick: u64,
    /// For decay — accumulated values attenuate without contact.
    pub last_contact_tick: u64,
    /// Accumulated from raids and conflicts. Decays over time.
    pub hostility: f32,
    /// Accumulated from trade and aid. Decays over time.
    pub cooperation: f32,
    /// Degree of concept and capability transmission that has occurred.
    pub cultural_exchange: f32,
    /// Active and recently broken formal agreements.
    pub agreements: Vec<CivAgreement>,
    /// Contact events ordered by tick. Pruned by salience.
    pub contact_log: Vec<CivContactEntry>,
}

/// Detailed structural metrics for a civilization.
/// Fully populated for focus civ and related civs; sparse for distant civs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CivilizationalMetrics {
    pub social_scale: MetricValue,
    pub administrative_complexity: MetricValue,
    pub territorial_coherence: MetricValue,
    pub specialization_index: MetricValue,
    pub surplus_capacity: MetricValue,
    pub ritual_specialization: MetricValue,
    pub leadership_concentration: MetricValue,
    pub redistribution_centrality: MetricValue,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Civilization {
    pub id: CivId,
    pub label: String,
    pub cohort_id: CohortId,
    /// Approximate geographic zone.
    pub location: RegionId,
    /// Disposition toward neighbors. Rising + high `resource_pressure` → raids.
    pub aggression: MetricValue,
    pub metrics: CivilizationalMetrics,
    /// Sparse — only civs with actual contact history.
    pub inter_civ_relations: BTreeMap<CivId, CivRelation>,
}

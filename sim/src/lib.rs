pub mod action;
pub mod genesis;
pub mod needs;
pub mod physical;
pub mod state;
pub mod tick;

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        assert!(true);
    }
}

#[cfg(test)]
mod phase2_tests {
    use std::collections::BTreeMap;

    use crate::state::action::ActionTag;
    use crate::state::agent::{
        Agent, AgentNeeds, AgentTraits, BeliefEntry, BondType, EventRef, KinRelation,
        KnowledgeState, MemoryEntry, Relationship, Sex,
    };
    use crate::state::capabilities::{Capability, DiscoveryMechanism};
    use crate::state::civ::{
        AgreementStatus, AgreementType, CivAgreement, CivContactEntry, CivContactType,
        CivilizationalMetrics, CivRelation, Civilization, ContactOutcome,
    };
    use crate::state::cohort::{
        AgeDistribution, Cohort, MetricField, MetricValue, NeedSatisfactionRates, PopulationState,
        TraitDistribution,
    };
    use crate::state::concepts::{
        Concept, ConceptType, EmergenceConditions, TransmissionMedium, TransmissionProfile,
        UtilityModifier,
    };
    use crate::state::ids::{
        AgentId, CapabilityId, CivId, CohortId, ConceptId, Domain, RegionId, TileId,
    };
    use crate::state::physical::{
        ClimateState, DiseaseVector, PhysicalWorld, RegenerationRates, ResourceLevels, Tile,
        TerrainType,
    };

    // ── ID types ───────────────────────────────────────────────────────────────

    #[test]
    fn id_newtype_ordering() {
        assert!(AgentId(1) < AgentId(2));
        assert!(CohortId(0) < CohortId(100));
        assert!(TileId(5) > TileId(3));
    }

    #[test]
    fn id_types_usable_as_btreemap_keys() {
        let mut m: BTreeMap<AgentId, u32> = BTreeMap::new();
        m.insert(AgentId(2), 20);
        m.insert(AgentId(1), 10);
        let keys: Vec<AgentId> = m.keys().copied().collect();
        assert_eq!(keys, vec![AgentId(1), AgentId(2)]);
    }

    #[test]
    fn domain_is_capability_id_alias() {
        let d: Domain = CapabilityId(42);
        assert_eq!(d, CapabilityId(42));
    }

    // ── KnowledgeState ordering ────────────────────────────────────────────────

    #[test]
    fn knowledge_state_ordering() {
        assert!(KnowledgeState::Ignorance < KnowledgeState::Misattribution);
        assert!(KnowledgeState::Misattribution < KnowledgeState::RuleWithoutUnderstanding);
        assert!(KnowledgeState::RuleWithoutUnderstanding < KnowledgeState::PartialUnderstanding);
        assert!(KnowledgeState::PartialUnderstanding < KnowledgeState::FullUnderstanding);
    }

    // ── Agent types ────────────────────────────────────────────────────────────

    #[test]
    fn agent_default_constructs() {
        let a = Agent::default();
        assert_eq!(a.id, AgentId(0));
        assert!(a.relationships.is_empty());
        assert!(a.personal_memory.is_empty());
        assert!(a.cultural_memory.is_empty());
        assert!(a.knowledge.is_empty());
    }

    #[test]
    fn agent_needs_default_constructs() {
        let n = AgentNeeds::default();
        assert_eq!(n.food, 0.0);
        assert_eq!(n.meaning, 0.0);
    }

    #[test]
    fn agent_traits_default_constructs() {
        let t = AgentTraits::default();
        assert_eq!(t.brave, 0.0);
        assert_eq!(t.tribal, 0.0);
    }

    #[test]
    fn sex_variants_accessible() {
        let _male = Sex::Male;
        let _female = Sex::Female;
    }

    #[test]
    fn relationship_constructs() {
        let r = Relationship {
            trust: 0.8,
            affection: 0.5,
            rivalry: 0.1,
            bond_type: BondType::Friendship,
            kin_relation: None,
            last_interaction_tick: 10,
        };
        assert_eq!(r.trust, 0.8);
    }

    #[test]
    fn relationship_with_kin_relation() {
        let r = Relationship {
            bond_type: BondType::Kin,
            kin_relation: Some(KinRelation::Parent),
            ..Relationship::default()
        };
        assert!(matches!(r.kin_relation, Some(KinRelation::Parent)));
    }

    #[test]
    fn event_ref_constructs() {
        let e = EventRef { tick: 5, event_id: 99 };
        assert_eq!(e.event_id, 99);
    }

    #[test]
    fn belief_entry_constructs() {
        let b = BeliefEntry {
            concept_id: ConceptId(1),
            strength: 0.7,
            generation_distance: 2,
            knowledge_state: KnowledgeState::PartialUnderstanding,
        };
        assert_eq!(b.strength, 0.7);
    }

    #[test]
    fn memory_entry_constructs() {
        let m = MemoryEntry {
            tick: 3,
            event: EventRef { tick: 3, event_id: 1 },
            salience: 0.9,
            interpretation: "it happened".to_string(),
        };
        assert_eq!(m.salience, 0.9);
    }

    #[test]
    fn agent_btreemap_relationships_ordered() {
        let mut a = Agent::default();
        a.relationships.insert(AgentId(3), Relationship::default());
        a.relationships.insert(AgentId(1), Relationship::default());
        let keys: Vec<AgentId> = a.relationships.keys().copied().collect();
        assert_eq!(keys, vec![AgentId(1), AgentId(3)]);
    }

    // ── Physical types ─────────────────────────────────────────────────────────

    #[test]
    fn tile_default_constructs() {
        let t = Tile::default();
        assert_eq!(t.id, TileId(0));
        assert!(matches!(t.terrain, TerrainType::Grassland));
    }

    #[test]
    fn resource_levels_constructs() {
        let r = ResourceLevels { food: 1.0, water: 0.8, stone: 0.0, wood: 0.5, metal: 0.0 };
        assert_eq!(r.food, 1.0);
    }

    #[test]
    fn regeneration_rates_constructs() {
        let r = RegenerationRates { food: 0.3, water: 0.5, stone: 0.01, wood: 0.2, metal: 0.0 };
        assert_eq!(r.food, 0.3);
    }

    #[test]
    fn physical_world_btreemap_tiles_ordered() {
        let mut w = PhysicalWorld::default();
        w.tiles.insert(TileId(5), Tile { id: TileId(5), ..Tile::default() });
        w.tiles.insert(TileId(1), Tile { id: TileId(1), ..Tile::default() });
        let keys: Vec<TileId> = w.tiles.keys().copied().collect();
        assert_eq!(keys, vec![TileId(1), TileId(5)]);
    }

    #[test]
    fn climate_state_default_constructs() {
        let c = ClimateState::default();
        assert_eq!(c.precipitation, 0.5);
    }

    #[test]
    fn disease_vector_default_constructs() {
        let d = DiseaseVector::default();
        assert!(d.active_tiles.is_empty());
    }

    // ── Cohort types ───────────────────────────────────────────────────────────

    #[test]
    fn cohort_default_constructs() {
        let c = Cohort::default();
        assert!(c.belief_profile.is_empty());
        assert!(c.capability_profile.is_empty());
        assert!(c.age_distribution.is_none());
    }

    #[test]
    fn metric_value_constructs() {
        let m = MetricValue { value: 0.6, velocity: 0.02 };
        assert_eq!(m.value, 0.6);
    }

    #[test]
    fn age_distribution_constructs() {
        let a = AgeDistribution { children: 0.3, adults: 0.6, elders: 0.1 };
        assert!((a.children + a.adults + a.elders - 1.0).abs() < 1e-6);
    }

    #[test]
    fn need_satisfaction_rates_constructs() {
        let n = NeedSatisfactionRates::default();
        assert_eq!(n.food, 0.0);
    }

    #[test]
    fn trait_distribution_btreemap_constructs() {
        let mut td = TraitDistribution::default();
        td.means.insert("brave".to_string(), 0.5);
        td.variances.insert("brave".to_string(), 0.1);
        assert_eq!(td.means["brave"], 0.5);
    }

    #[test]
    fn population_state_constructs() {
        let p = PopulationState { count: 8, growth_rate: 0.01 };
        assert_eq!(p.count, 8);
    }

    #[test]
    fn metric_field_variants_accessible() {
        let _ = MetricField::SocialScale;
        let _ = MetricField::Cohesion;
        let _ = MetricField::ResourcePressure;
    }

    // ── Civilization types ─────────────────────────────────────────────────────

    #[test]
    fn civilization_default_constructs() {
        let c = Civilization::default();
        assert!(c.inter_civ_relations.is_empty());
    }

    #[test]
    fn civ_relation_default_constructs() {
        let r = CivRelation::default();
        assert!(r.agreements.is_empty());
        assert!(r.contact_log.is_empty());
    }

    #[test]
    fn civ_agreement_constructs() {
        let a = CivAgreement {
            agreement_type: AgreementType::NonAggression,
            formed_tick: 10,
            status: AgreementStatus::Active,
        };
        assert!(matches!(a.status, AgreementStatus::Active));
    }

    #[test]
    fn agreement_status_broken_variant() {
        let s = AgreementStatus::Broken { by: CivId(2), at_tick: 50 };
        if let AgreementStatus::Broken { by, at_tick } = s {
            assert_eq!(by, CivId(2));
            assert_eq!(at_tick, 50);
        } else {
            panic!("expected Broken variant");
        }
    }

    #[test]
    fn civ_contact_entry_constructs() {
        let e = CivContactEntry {
            tick: 100,
            contact_type: CivContactType::Trade,
            initiator: CivId(1),
            outcome: ContactOutcome::Success,
            salience: 0.4,
        };
        assert_eq!(e.tick, 100);
    }

    #[test]
    fn civilizational_metrics_default_constructs() {
        let m = CivilizationalMetrics::default();
        assert_eq!(m.social_scale.value, 0.0);
    }

    // ── Concept types ──────────────────────────────────────────────────────────

    #[test]
    fn concept_constructs() {
        let c = Concept {
            id: ConceptId(1),
            label: "fire".to_string(),
            concept_type: ConceptType::CausalModel,
            utility_modifiers: vec![],
            transmission: TransmissionProfile::default(),
            emergence_conditions: EmergenceConditions::default(),
            conflicts_with: vec![],
        };
        assert_eq!(c.label, "fire");
    }

    #[test]
    fn utility_modifier_constructs() {
        let m = UtilityModifier {
            action_tag: ActionTag::Forage,
            direction: 0.3,
            threshold: 0.5,
        };
        assert_eq!(m.direction, 0.3);
    }

    #[test]
    fn transmission_medium_variants() {
        let _ = TransmissionMedium::Oral;
        let _ = TransmissionMedium::Written;
        let _ = TransmissionMedium::Ritual;
        let _ = TransmissionMedium::DirectObservation;
    }

    #[test]
    fn emergence_conditions_default_constructs() {
        let e = EmergenceConditions::default();
        assert!(e.metric_thresholds.is_empty());
        assert!(!e.player_intervention);
    }

    // ── Capability types ───────────────────────────────────────────────────────

    #[test]
    fn capability_constructs() {
        let c = Capability {
            id: CapabilityId(1),
            label: "fire_starting".to_string(),
            prerequisite_capabilities: vec![],
            prerequisite_concepts: vec![],
            metric_thresholds: vec![],
            unlocked_actions: vec![ActionTag::Forage],
            unlocks_concepts: vec![],
            discovery_mechanism: DiscoveryMechanism::TrialAndError,
        };
        assert_eq!(c.label, "fire_starting");
    }

    #[test]
    fn discovery_mechanism_variants() {
        let _ = DiscoveryMechanism::Observation;
        let _ = DiscoveryMechanism::TrialAndError;
        let _ = DiscoveryMechanism::Transmission;
        let _ = DiscoveryMechanism::DivineGnosis;
    }

    // ── ActionTag ──────────────────────────────────────────────────────────────

    #[test]
    fn action_tag_variants_accessible() {
        let _ = ActionTag::Forage;
        let _ = ActionTag::Hunt;
        let _ = ActionTag::Rest;
        let _ = ActionTag::Socialize;
        let _ = ActionTag::Redistribute;
    }

    // ── WorldState types ───────────────────────────────────────────────────────

    #[test]
    fn world_clock_default() {
        use crate::state::world::WorldClock;
        let c = WorldClock::default();
        assert_eq!(c.year, -300_000);
        assert_eq!(c.tick, 0);
    }

    #[test]
    fn agent_archive_default_empty() {
        use crate::state::world::AgentArchive;
        let a = AgentArchive::default();
        assert!(a.agents.is_empty());
    }

    #[test]
    fn agent_id_default_is_zero() {
        assert_eq!(AgentId::default(), AgentId(0));
    }

    #[test]
    fn region_id_usable_as_field() {
        let _r = RegionId(7);
        let _cohort_id = CohortId(1);
    }
}

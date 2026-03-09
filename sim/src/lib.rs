pub mod action;
pub mod genesis;
pub mod needs;
pub mod physical;
pub mod state;
pub mod tick;

pub use genesis::genesis;

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
        AgreementStatus, AgreementType, CivAgreement, CivContactEntry, CivContactType, CivRelation,
        Civilization, CivilizationalMetrics, ContactOutcome,
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
        ClimateState, DiseaseVector, PhysicalWorld, RegenerationRates, ResourceLevels, TerrainType,
        Tile,
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
        let e = EventRef {
            tick: 5,
            event_id: 99,
        };
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
            event: EventRef {
                tick: 3,
                event_id: 1,
            },
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
        let r = ResourceLevels {
            food: 1.0,
            water: 0.8,
            stone: 0.0,
            wood: 0.5,
            metal: 0.0,
        };
        assert_eq!(r.food, 1.0);
    }

    #[test]
    fn regeneration_rates_constructs() {
        let r = RegenerationRates {
            food: 0.3,
            water: 0.5,
            stone: 0.01,
            wood: 0.2,
            metal: 0.0,
        };
        assert_eq!(r.food, 0.3);
    }

    #[test]
    fn physical_world_btreemap_tiles_ordered() {
        let mut w = PhysicalWorld::default();
        w.tiles.insert(
            TileId(5),
            Tile {
                id: TileId(5),
                ..Tile::default()
            },
        );
        w.tiles.insert(
            TileId(1),
            Tile {
                id: TileId(1),
                ..Tile::default()
            },
        );
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
        let m = MetricValue {
            value: 0.6,
            velocity: 0.02,
        };
        assert_eq!(m.value, 0.6);
    }

    #[test]
    fn age_distribution_constructs() {
        let a = AgeDistribution {
            children: 0.3,
            adults: 0.6,
            elders: 0.1,
        };
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
        let p = PopulationState {
            count: 8,
            growth_rate: 0.01,
        };
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
        let s = AgreementStatus::Broken {
            by: CivId(2),
            at_tick: 50,
        };
        let AgreementStatus::Broken { by, at_tick } = s else {
            panic!("expected Broken variant");
        };
        assert_eq!(by, CivId(2));
        assert_eq!(at_tick, 50);
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

#[cfg(test)]
mod phase3_tests {
    use crate::genesis;

    #[test]
    fn genesis_completes_without_panic() {
        let _ = genesis(42);
    }

    #[test]
    fn genesis_same_seed_identical_output() {
        let a = genesis(42);
        let b = genesis(42);
        let json_a = serde_json::to_string(&a).unwrap();
        let json_b = serde_json::to_string(&b).unwrap();
        assert_eq!(json_a, json_b);
    }

    #[test]
    fn genesis_different_seeds_diverge() {
        let a = genesis(42);
        let b = genesis(43);
        // Trait values must differ between seeds.
        let traits_a: Vec<f32> = a.agents.values().map(|ag| ag.traits.brave).collect();
        let traits_b: Vec<f32> = b.agents.values().map(|ag| ag.traits.brave).collect();
        assert_ne!(
            traits_a, traits_b,
            "seeds 42 and 43 should produce different trait values"
        );
    }

    #[test]
    fn all_agents_reference_valid_cohort() {
        let world = genesis(42);
        for (id, agent) in &world.agents {
            assert!(
                world.cohorts.contains_key(&agent.cohort_id),
                "agent {id:?} references missing cohort {:?}",
                agent.cohort_id
            );
        }
    }

    #[test]
    fn agent_count_matches_cohort_population() {
        let world = genesis(42);
        let agent_count = world.agents.len() as u32;
        for cohort in world.cohorts.values() {
            assert_eq!(
                cohort.population.count, agent_count,
                "cohort population.count {} != agent count {}",
                cohort.population.count, agent_count
            );
        }
    }

    #[test]
    fn tile_grid_has_25_tiles() {
        let world = genesis(42);
        assert_eq!(world.world.tiles.len(), 25);
    }

    #[test]
    fn tile_ids_are_1_through_25() {
        use crate::state::ids::TileId;
        let world = genesis(42);
        let ids: Vec<TileId> = world.world.tiles.keys().copied().collect();
        let expected: Vec<TileId> = (1..=25).map(TileId).collect();
        assert_eq!(ids, expected);
    }

    #[test]
    fn central_tiles_are_grassland() {
        use crate::state::ids::TileId;
        use crate::state::physical::TerrainType;
        let world = genesis(42);
        // Central 3×3: tile ids 7,8,9, 12,13,14, 17,18,19
        for &id in &[7u64, 8, 9, 12, 13, 14, 17, 18, 19] {
            let tile = world.world.tiles.get(&TileId(id)).unwrap();
            assert!(
                matches!(tile.terrain, TerrainType::Grassland),
                "tile {id} should be Grassland"
            );
        }
    }

    #[test]
    fn all_agents_start_at_central_tile() {
        use crate::state::ids::TileId;
        let world = genesis(42);
        for (id, agent) in &world.agents {
            assert_eq!(
                agent.location,
                TileId(13),
                "agent {id:?} should start on central tile"
            );
        }
    }

    #[test]
    fn all_agent_needs_initialised_at_0_9() {
        let world = genesis(42);
        for (id, agent) in &world.agents {
            let n = &agent.needs;
            for &v in &[
                n.food,
                n.water,
                n.sleep,
                n.shelter,
                n.warmth,
                n.safety,
                n.belonging,
                n.status,
                n.meaning,
            ] {
                assert!(
                    (v - 0.9).abs() < 1e-6,
                    "agent {id:?} need should be 0.9, got {v}"
                );
            }
        }
    }

    #[test]
    fn clock_initialised_correctly() {
        let world = genesis(42);
        assert_eq!(world.clock.year, -300_000);
        assert_eq!(world.clock.tick, 0);
        assert_eq!(world.clock.last_delta, 0.0);
    }

    #[test]
    fn seed_stored_in_world_state() {
        let world = genesis(42);
        assert_eq!(world.seed, 42);
    }
}

#[cfg(test)]
mod phase4_tests {
    use crate::physical::physical_update;
    use crate::state::ids::TileId;
    use crate::state::physical::{
        PhysicalWorld, RegenerationRates, ResourceLevels, TerrainType, Tile,
    };

    fn grassland_tile(food: f32) -> Tile {
        Tile {
            id: TileId(1),
            terrain: TerrainType::Grassland,
            resources: ResourceLevels {
                food,
                ..ResourceLevels::default()
            },
            resource_regeneration: RegenerationRates {
                food: 0.3,
                ..RegenerationRates::default()
            },
            resource_max: ResourceLevels {
                food: 1.0,
                ..ResourceLevels::default()
            },
            ..Tile::default()
        }
    }

    fn world_with_tile(tile: Tile) -> PhysicalWorld {
        let mut world = PhysicalWorld::default();
        world.tiles.insert(tile.id, tile);
        world
    }

    #[test]
    fn grassland_food_regenerates_over_one_year() {
        let mut world = world_with_tile(grassland_tile(0.5));
        physical_update(&mut world, 1.0);
        let food = world.tiles[&TileId(1)].resources.food;
        // Expected: 0.5 + 0.3 * 1.0 = 0.8
        assert!((food - 0.8).abs() < 1e-5, "expected 0.8, got {food}");
    }

    #[test]
    fn regeneration_is_linear_in_delta_t() {
        // delta_t=10 should regenerate exactly 10× as much as delta_t=1.
        // resource_max is set high enough (10.0) so the cap is never hit during this test.
        let uncapped_tile = |food: f32| Tile {
            id: TileId(1),
            terrain: TerrainType::Grassland,
            resources: ResourceLevels {
                food,
                ..ResourceLevels::default()
            },
            resource_regeneration: RegenerationRates {
                food: 0.3,
                ..RegenerationRates::default()
            },
            resource_max: ResourceLevels {
                food: 10.0, // high ceiling so neither delta_t hits it
                ..ResourceLevels::default()
            },
            ..Tile::default()
        };

        let mut world1 = world_with_tile(uncapped_tile(0.0));
        physical_update(&mut world1, 1.0);
        let gain1 = world1.tiles[&TileId(1)].resources.food;

        let mut world10 = world_with_tile(uncapped_tile(0.0));
        physical_update(&mut world10, 10.0);
        let gain10 = world10.tiles[&TileId(1)].resources.food;

        assert!(
            (gain10 - gain1 * 10.0).abs() < 1e-4,
            "10-year gain ({gain10}) should be 10× 1-year gain ({gain1})"
        );
    }

    #[test]
    fn resource_never_exceeds_baseline_max() {
        // Start at 0.9 with regen 0.3 over delta_t=10 — would overshoot without clamping.
        let mut world = world_with_tile(grassland_tile(0.9));
        physical_update(&mut world, 10.0);
        let food = world.tiles[&TileId(1)].resources.food;
        assert!(
            food <= 1.0,
            "food ({food}) must not exceed baseline_max 1.0"
        );
    }

    #[test]
    fn resource_never_goes_negative() {
        // Tile with zero regen — food should stay at 0.0, not drop negative.
        let tile = Tile {
            id: TileId(1),
            terrain: TerrainType::Mountain,
            resources: ResourceLevels::default(), // all zero
            resource_regeneration: RegenerationRates::default(), // all zero
            resource_max: ResourceLevels::default(),
            ..Tile::default()
        };
        let mut world = world_with_tile(tile);
        physical_update(&mut world, 1.0);
        let food = world.tiles[&TileId(1)].resources.food;
        assert!(food >= 0.0, "food ({food}) must not be negative");
    }

    #[test]
    fn physical_update_with_no_agents_completes_without_panic() {
        // genesis produces a world with agents; physical_update only touches tiles.
        use crate::genesis;
        let mut world = genesis(42);
        physical_update(&mut world.world, 1.0); // should not panic
    }
}

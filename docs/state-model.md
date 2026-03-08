# State Model

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Contents**

- [WorldState](#worldstate)
- [WorldClock](#worldclock)
- [ID Types](#id-types)
- [AgentArchive](#agentarchive)
- [Agent (Tier 1)](#agent-tier-1)
  - [Identity](#identity)
    - [Sex](#sex)
  - [Needs](#needs)
  - [Traits](#traits)
  - [Relationships](#relationships)
    - [Relationship](#relationship)
    - [BondType](#bondtype)
    - [KinRelation](#kinrelation)
  - [Memory](#memory)
    - [MemoryEntry](#memoryentry)
    - [EventRef](#eventref)
    - [BeliefEntry](#beliefentry)
    - [KnowledgeState](#knowledgestate)
  - [Knowledge](#knowledge)
- [Concept Registry](#concept-registry)
  - [Concept](#concept)
  - [ConceptType](#concepttype)
  - [UtilityModifier](#utilitymodifier)
  - [TransmissionProfile](#transmissionprofile)
    - [TransmissionMedium](#transmissionmedium)
  - [EmergenceConditions](#emergenceconditions)
- [Capability Graph](#capability-graph)
  - [Capability](#capability)
  - [DiscoveryMechanism](#discoverymechanism)
  - [One DAG, two node types](#one-dag-two-node-types)
- [ActionTag](#actiontag)
- [Cohort](#cohort)
  - [AgeDistribution](#agedistribution)
  - [NeedSatisfactionRates](#needsatisfactionrates)
  - [TraitDistribution](#traitdistribution)
- [Civilization](#civilization)
- [CivRelation](#civrelation)
  - [CivContactEntry](#civcontactentry)
  - [CivAgreement](#civagreement)
  - [CivContactType](#civcontacttype)
  - [ContactOutcome](#contactoutcome)
- [PhysicalWorld](#physicalworld)
  - [ClimateState](#climatestate)
  - [DiseaseVector](#diseasevector)
  - [Tile](#tile)
    - [TerrainType](#terraintype)
    - [ResourceLevels](#resourcelevels)
    - [RegenerationRates](#regenerationrates)
- [CivilizationalMetrics](#civilizationalmetrics)
  - [MetricField](#metricfield)
  - [MetricValue](#metricvalue)
  - [PopulationState](#populationstate)
  - [Why velocity is load-bearing](#why-velocity-is-load-bearing)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

Concrete specification of what the simulation stores. For the conceptual design behind these fields, see [simulation-entities.md](simulation-entities.md). For how state transitions work, see [architecture.md](architecture.md).

All continuous values are `f32` in `[0.0, 1.0]` unless noted. Need satisfaction values use: `0.0` = completely unmet / critical, `1.0` = fully satisfied. They decay exponentially over time; agents act to restore them.

All agent collections use `BTreeMap` keyed by ID to guarantee deterministic iteration order. See architecture.md — Agent Layer Determinism.

---

## WorldState

The top-level container. Serialized in full for save/load and determinism replay.

| Field | Type | Notes |
|---|---|---|
| `seed` | `u64` | Original PRNG seed. Serialized with state. |
| `rng` | `ChaChaRng` | Current PRNG state. Deterministic. |
| `clock` | `WorldClock` | Current time and tick count. |
| `agents` | `BTreeMap<AgentId, Agent>` | Active named agents with individual tracking. |
| `cohorts` | `BTreeMap<CohortId, Cohort>` | All cohorts at variable fidelity. Focus civ and related civs run full pipeline; distant civs run sparse pipeline. Same type, different depth. |
| `civilizations` | `BTreeMap<CivId, Civilization>` | All civilizations including the focus civ. Each holds a `CohortId` reference and its own `CivilizationalMetrics`. |
| `focus_civ_id` | `CivId` | Which civilization is the player's. |
| `world` | `PhysicalWorld` | Terrain, climate, resources, disease. |
| `archive` | `AgentArchive` | Immutable. Dead agents only. Nothing writes here after death. |
| `concepts` | `BTreeMap<ConceptId, Concept>` | Definition registry for all concepts that exist anywhere in the simulation. |
| `capabilities` | `BTreeMap<CapabilityId, Capability>` | Definition registry for all capabilities that have been discovered anywhere. |

---

## WorldClock

| Field | Type | Notes |
|---|---|---|
| `year` | `i64` | Negative = BCE. Starts around -300,000. |
| `tick` | `u64` | Monotonically increasing tick counter. |
| `last_delta` | `f32` | Years elapsed in the most recent tick. |

---

## ID Types

All ID types are `u64` newtypes. Stable across the lifetime of the entity they identify. Never reused after entity death or removal.

```
AgentId:      u64
CohortId:     u64
CivId:        u64
ConceptId:    u64
CapabilityId: u64
TileId:       u64
RegionId:     u64
Domain:       CapabilityId   // alias; used in Agent.knowledge to key per-capability mastery levels
```

---

## AgentArchive

Immutable store for dead agents. Written once at death; never modified after.

| Field | Type | Notes |
|---|---|---|
| `agents` | `BTreeMap<AgentId, Agent>` | Keyed by id. Entry is sealed at the death tick. |

---

## Agent (Tier 1)

Full-fidelity named agent.

### Identity

| Field | Type | Notes |
|---|---|---|
| `id` | `AgentId` | Stable across lifetime. |
| `name` | `String` | |
| `age` | `f32` | Years. |
| `sex` | `Sex` | Biological sex. Affects reproduction only. |
| `location` | `TileId` | Current position in physical world. |
| `cohort_id` | `CohortId` | The cohort this agent belongs to. They are a member of that population; individual tracking is the only thing that distinguishes them from unnamed members. |

#### Sex

```
Sex:
    Male
    Female
```

### Needs

Each need is a satisfaction level decaying toward 0 over time. Rates differ per need.

| Field | Type | Decay character |
|---|---|---|
| `food` | `f32` | Fast — days to critical |
| `water` | `f32` | Very fast — hours to critical |
| `sleep` | `f32` | Fast — days to critical |
| `shelter` | `f32` | Slow — weeks; climate-modulated |
| `warmth` | `f32` | Climate-dependent rate |
| `safety` | `f32` | Situational; spikes and recovers |
| `belonging` | `f32` | Slow — weeks to months |
| `status` | `f32` | Slow; event-driven changes |
| `meaning` | `f32` | Very slow — months to years |

### Traits

Stable dispositional values. Inherited with mutation at birth. Influence utility weight curves, not hard behavior rules.

| Field | Type | Notes |
|---|---|---|
| `brave` | `f32` | High = more willing to accept physical risk |
| `cautious` | `f32` | High = prefers known options; resists novelty |
| `aggressive` | `f32` | High = more likely to select conflict actions |
| `empathetic` | `f32` | High = other agents' need states influence own utility |
| `curious` | `f32` | High = bonus utility for exploration and discovery actions |
| `credulous` | `f32` | High = faster belief adoption, lower evidence threshold |
| `charismatic` | `f32` | High = more influence on others' belief adoption |
| `dominant` | `f32` | High = seeks high-status actions; poor fit for deference |
| `deferential` | `f32` | High = accepts authority easily; stable in hierarchies |
| `compassionate` | `f32` | High = meaning need partially satisfied by helping others |
| `tribal` | `f32` | High = strong in-group preference; poor out-group trust |

Traits are not a fixed enum — the list will grow. Avoid building systems that assume a fixed trait count.

### Relationships

| Field | Type | Notes |
|---|---|---|
| `relationships` | `BTreeMap<AgentId, Relationship>` | Sparse — only known agents. |

#### Relationship

| Field | Type | Notes |
|---|---|---|
| `trust` | `f32` | General reliability. Updates from shared experience. |
| `affection` | `f32` | Warmth and closeness. |
| `rivalry` | `f32` | Active competition. Can coexist with affection. |
| `bond_type` | `BondType` | `Kin`, `Friendship`, `Romantic`, `Hierarchy`, `Institutional` |
| `kin_relation` | `Option<KinRelation>` | `Parent`, `Child`, `Sibling`, `Cousin`, etc. Null if not kin. |
| `last_interaction_tick` | `u64` | For attenuation — relationships decay without contact. |

#### BondType

```
BondType:
    Kin             // blood or adoptive family
    Friendship      // chosen social bond
    Romantic        // pair bond; may overlap with Kin after children
    Hierarchy       // authority relationship: leader/follower, patron/client
    Institutional   // role-defined: fellow guild member, co-religionist, fellow soldier
```

#### KinRelation

Present only when `bond_type` is `Kin`. Describes the specific family relationship from the perspective of the agent who holds the record.

```
KinRelation:
    Parent
    Child
    Sibling
    Grandparent
    Grandchild
    Cousin
    AuntOrUncle
    NieceOrNephew
    Spouse          // kin by marriage; may also carry Romantic bond_type
```

### Memory

| Field | Type | Notes |
|---|---|---|
| `personal_memory` | `Vec<MemoryEntry>` | Ordered by recency. Pruned by emotional salience over time. |
| `cultural_memory` | `BTreeMap<ConceptId, BeliefEntry>` | What the agent believes about the world. |

#### MemoryEntry

| Field | Type | Notes |
|---|---|---|
| `tick` | `u64` | When it happened. |
| `event` | `EventRef` | Reference to the archived event. |
| `salience` | `f32` | Emotional weight. High-salience memories decay slower. |
| `interpretation` | `String` | The agent's causal attribution at time of encoding. |

#### EventRef

Opaque reference into the global event log. The referenced event is immutable once archived.

```
EventRef {
    tick:     u64,   // tick at which the event occurred
    event_id: u64,   // stable identifier within the event log
}
```

#### BeliefEntry

| Field | Type | Notes |
|---|---|---|
| `concept_id` | `ConceptId` | What the belief is about — a deity, a causal rule, a taboo. |
| `strength` | `f32` | Conviction. Decays without reinforcement. |
| `generation_distance` | `u32` | Hops from original source. Increases with each transmission. Drives epistemic decay. |
| `knowledge_state` | `KnowledgeState` | `Ignorance`, `Misattribution`, `RuleWithoutUnderstanding`, `PartialUnderstanding`, `FullUnderstanding` |

#### KnowledgeState

Represents how accurately an agent or cohort understands a domain or concept. Ordered from least to most accurate. Transmission and decay operate on this scale.

```
KnowledgeState:
    Ignorance                // unaware the domain or concept exists
    Misattribution           // has a causal model, but it is wrong (e.g., disease from bad air)
    RuleWithoutUnderstanding // knows what to do but not why; fragile under novel conditions
    PartialUnderstanding     // accurate on part of the causal chain; can improve with experience
    FullUnderstanding        // accurate, complete causal model; can innovate and teach reliably
```

### Knowledge

Distinct from belief. Empirically-grounded capability or causal understanding.

| Field | Type | Notes |
|---|---|---|
| `knowledge` | `BTreeMap<Domain, KnowledgeState>` | Per domain: medicine, agriculture, metallurgy, etc. |

---

## Concept Registry

`ConceptId` is used throughout the model as a key — in agent `cultural_memory`, cohort `belief_profile`, Tier 3 `dominant_belief_profile`, and agent `knowledge`. The registry is the world-level record of what each `ConceptId` actually is and what it does.

The registry lives in `WorldState.concepts`. It contains **definitions only** — what a concept is, what it does, how it spreads. It does not track who holds what. Whether a specific agent or civilization holds a concept is tracked in agent `cultural_memory` and cohort `belief_profile`.

The registry is not pre-loaded. Concepts come into existence dynamically — through organic cultural evolution, player intervention, or cross-civilization transmission. A prehistoric band has no concept of communism; the registry entry doesn't exist yet.

`Domain` (used in agent knowledge) is an alias for `CapabilityId`. Agent knowledge tracks mastery of capabilities, not general beliefs — those live in `cultural_memory`.

### Concept

| Field | Type | Notes |
|---|---|---|
| `id` | `ConceptId` | Stable identifier. |
| `label` | `String` | Human-readable name. e.g., "Asha", "communism", "germ theory", "cattle sacrifice taboo" |
| `concept_type` | `ConceptType` | See below. |
| `utility_modifiers` | `Vec<UtilityModifier>` | Behavioral effects on agents who hold this belief. |
| `transmission` | `TransmissionProfile` | How it spreads and mutates. |
| `emergence_conditions` | `EmergenceConditions` | What must be true for this concept to come into existence. |
| `conflicts_with` | `Vec<ConceptId>` | Concepts this one competes with. Holding both creates tension; one tends to displace the other. |

### ConceptType

```
ConceptType:
    Deity             // a named supernatural agent; may accumulate theology
    Ideology          // political/social/economic framework; shapes collective behavior
    CausalModel       // a claim about how the world works; right or wrong
    Taboo             // a behavioral prohibition; follows rule-without-understanding dynamics
    Institution       // a formal role structure: priesthood, law, market, army
    NaturalPhenomenon // an agentive interpretation of a natural thing: thunder-being, river spirit
```

### UtilityModifier

Defines how holding a concept at a given strength modifies action utility scores. Applied during action selection for agents whose belief strength exceeds the threshold.

```
UtilityModifier {
    action_tag:  ActionTag,  // which class of actions this affects
    direction:   f32,        // positive = bonus, negative = penalty; magnitude scales with belief strength
    threshold:   f32,        // minimum belief strength for this modifier to apply
}
```

Examples for a communism-like ideology:
- `{ action_tag: Redistribute, direction: +0.4, threshold: 0.3 }`
- `{ action_tag: HoardResources, direction: -0.3, threshold: 0.5 }`
- `{ action_tag: DeferToHierarchy, direction: -0.2, threshold: 0.6 }`

### TransmissionProfile

| Field | Type | Notes |
|---|---|---|
| `base_rate` | `f32` | Probability of transmission per social contact per year. |
| `mutation_rate` | `f32` | Probability that a transmitted copy drifts from the original. |
| `required_medium` | `TransmissionMedium` | `Oral`, `Written`, `Ritual`, `DirectObservation`. Oral is lossy; written is stable. |
| `charisma_amplified` | `bool` | Whether high-charisma transmitters dramatically increase spread rate. |

#### TransmissionMedium

```
TransmissionMedium:
    Oral              // spoken transmission; lossy, mutates with distance and time
    Written           // text-based; stable but requires literacy capability
    Ritual            // embodied practice; stable form, but meaning drifts without Written backup
    DirectObservation // first-hand witness; highest initial strength, not directly re-transmissible
```

### EmergenceConditions

What must be true **in the local civilization** for this concept to come into existence. Always checked against the specific civilization's state — not global. A concept existing in one civilization does not make it available in another; it must be transmitted explicitly or discovered independently.

```
EmergenceConditions {
    metric_thresholds:    Vec<(MetricField, f32)>,  // e.g., administrative_complexity > 0.6
    required_concepts:    Vec<ConceptId>,           // prerequisite ideas that must already exist
    population_minimum:   Option<u32>,              // some ideas require critical mass
    player_intervention:  bool,                     // if true, can only enter via divine action
}
```

Communism as an example — approximate emergence conditions:
- `administrative_complexity > 0.7` — visible bureaucratic hierarchy to react against
- `specialization_index > 0.6` — class stratification must be legible
- `surplus_capacity > 0.5` — enough surplus that distribution is a meaningful political question
- requires concepts: some prior notion of collective ownership or redistribution (even proto-form)
- `population_minimum: 500` — needs enough people for political abstraction to be socially useful

---

## Capability Graph

The world-level definition registry for capabilities. Lives in `WorldState.capabilities`. Contains **definitions only** — prerequisites, effects, discovery mechanisms. Does not track who has what.

Whether a capability is held by a specific agent is tracked in `Agent.knowledge`. Whether a cohort collectively holds it is tracked in `Cohort.capability_profile`. Prerequisite checks for emergence are always against the **local civilization's** state — a capability existing in one civilization does not make it available in another.

`Domain` in agent knowledge is an alias for `CapabilityId`. An agent's `KnowledgeState` for a given capability represents their individual mastery of it, independent of what the world registry says.

### Capability

| Field | Type | Notes |
|---|---|---|
| `id` | `CapabilityId` | Stable identifier. |
| `label` | `String` | e.g., "fire_starting", "agriculture", "iron_smelting", "writing" |
| `prerequisite_capabilities` | `Vec<CapabilityId>` | Must be discovered before this one is possible. |
| `prerequisite_concepts` | `Vec<ConceptId>` | Concepts that must exist before this can emerge. |
| `metric_thresholds` | `Vec<(MetricField, f32)>` | Civilizational conditions required. |
| `unlocked_actions` | `Vec<ActionTag>` | Actions that become available once this capability exists in the world. |
| `unlocks_concepts` | `Vec<ConceptId>` | Concepts that can now emerge once this capability exists. |
| `discovery_mechanism` | `DiscoveryMechanism` | How it enters the world. |

### DiscoveryMechanism

```
DiscoveryMechanism:
    Observation       // agents discover it by observing the physical world
    TrialAndError     // probabilistic discovery through repeated relevant action
    Transmission      // received from another civilization
    DivineGnosis      // player-granted; bypasses prerequisites
```

### One DAG, two node types

Capabilities and concepts form a single DAG. Nodes are of two types — `Capability` and `Concept` — with typed edges between them. As subgraphs, capabilities-only and concepts-only are each DAGs, and the cross-type edges don't introduce cycles, so the whole graph is a DAG.

They're stored in separate registries because the node types carry different fields and behave differently, not because they form separate graphs.

**Capability → Concept edges** (`unlocks_concepts`): a capability's existence enables concept emergence.
- Writing → stable theology, formal law, historical record as concept
- Iron smelting → warrior-caste ideology, imperial ambition as accessible concepts
- Surplus agriculture → property ownership, debt, redistribution ideology

**Concept → Capability edges** (`prerequisite_concepts`): a concept must exist for a capability to emerge.
- A deity of the forge may be required for iron smelting in some cultures (not universally — this is configurable per world)
- Formal measurement concepts may be required for advanced architecture
- A taboo can function as a negative prerequisite — blocking capability development even when physical conditions are met

**Both reference `ActionTag`**: capabilities unlock actions; concept `UtilityModifier`s change their utility. The same action can be enabled by a capability and amplified or suppressed by a belief. These work independently and compose.

---

## ActionTag

Classifies actions for the purpose of utility modification and capability gating. `UtilityModifier.action_tag` and `Capability.unlocked_actions` both reference these tags. The list grows as capabilities and concepts expand.

```
ActionTag:
    // Subsistence
    Forage
    Hunt
    Fish
    Farm

    // Resource management
    ShareResources
    Redistribute
    HoardResources
    Trade

    // Conflict
    Raid
    Defend
    Flee

    // Leadership and social
    DeferToHierarchy
    AssertDominance
    Negotiate

    // Cultural and epistemic
    TeachConcept
    PerformRitual
    Explore
    Innovate
```

Like traits, this list is open-ended. Avoid building systems that assume a fixed action count.

---

## Cohort

Represents the full population of a group — named agents included. Named agents who belong to this cohort have individual `Agent` records and are simulated at full fidelity; their states contribute to the cohort's aggregate fields each tick. Everyone else is simulated only at the aggregate level. `population.count` covers everyone.

The same struct is used for all cohorts. **Pipeline depth varies by fidelity level.** A cohort for the focus civilization or a group in direct contact runs the full tick pipeline. A cohort for a distant civilization with no relationship to the focus runs only the sparse pipeline — population dynamics, resource pressure, event threshold checks. The rich aggregate fields are empty for sparse cohorts and populated as a group becomes relevant.

**Warming up a sparse cohort:** when a distant civilization makes contact, its cohort's rich fields are populated by sampling from the sparse summary fields (`cohesion`, `resource_pressure`, `capability_level`) to generate plausible initial distributions. From that point forward, it runs the full pipeline.

| Field | Type | Notes |
|---|---|---|
| `id` | `CohortId` | |
| `label` | `String` | e.g., "Rowan's tribe" |
| `population` | `PopulationState` | Total headcount including individually-tracked members. All cohorts. |
| `cohesion` | `MetricValue` | Internal unity. Falling → fragmentation events. All cohorts. |
| `resource_pressure` | `MetricValue` | Food and land stress relative to population. All cohorts. |
| `capability_level` | `MetricValue` | Rough proxy for military and economic capacity relative to neighbors. All cohorts. |
| `age_distribution` | `Option<AgeDistribution>` | Rough breakdown: children, adults, elders. Full-pipeline only. |
| `need_satisfaction` | `Option<NeedSatisfactionRates>` | Aggregate satisfaction rates per need. Full-pipeline only. |
| `trait_distribution` | `Option<TraitDistribution>` | Mean and variance per trait across cohort. Full-pipeline only. |
| `belief_profile` | `BTreeMap<ConceptId, f32>` | Aggregate belief strength per concept. Sparse for distant civs; full for related ones. |
| `capability_profile` | `BTreeMap<CapabilityId, f32>` | Aggregate mastery level per capability. Sparse for distant civs; full for related ones. |
| `location` | `TileId` | Centroid or primary tile. |
| `affiliation` | `Option<AgentId>` | Named leader if one has emerged. Full-pipeline only. |

### AgeDistribution

Rough demographic breakdown. Full-pipeline only; `None` for sparse cohorts.

```
AgeDistribution {
    children: f32,   // fraction of population under ~15 years
    adults:   f32,   // fraction ~15–60 years
    elders:   f32,   // fraction over ~60 years
    // fractions sum to 1.0
}
```

### NeedSatisfactionRates

Aggregate need satisfaction across the cohort. Mirrors the per-need fields on `Agent.needs`, averaged over all members. Full-pipeline only.

```
NeedSatisfactionRates {
    food:      f32,
    water:     f32,
    sleep:     f32,
    shelter:   f32,
    warmth:    f32,
    safety:    f32,
    belonging: f32,
    status:    f32,
    meaning:   f32,
}
```

### TraitDistribution

Mean and variance per trait across the cohort. Full-pipeline only.

```
TraitDistribution {
    means:     BTreeMap<String, f32>,   // mean trait value across cohort members
    variances: BTreeMap<String, f32>,   // variance per trait; high variance = diverse cohort
}
```

---

## Civilization

Every civilization — including the focus civilization — has a `Civilization` record paired with a `Cohort`. The `Civilization` record holds identity and civilizational-level metrics. The `Cohort` holds population and demographic state.

| Field | Type | Notes |
|---|---|---|
| `id` | `CivId` | |
| `label` | `String` | |
| `cohort_id` | `CohortId` | The associated cohort. |
| `location` | `RegionId` | Approximate geographic zone. More precise location is on the Cohort. |
| `aggression` | `MetricValue` | Disposition toward neighbors. Rising + high `resource_pressure` → raids. |
| `metrics` | `CivilizationalMetrics` | Detailed structural metrics. Fully populated for focus civ and related civs; sparse for distant civs. |
| `inter_civ_relations` | `BTreeMap<CivId, CivRelation>` | Sparse — only civs with actual contact history. |

---

## CivRelation

Bilateral record between two civilizations. Only exists for pairs that have made actual contact — absent entry means no known contact, not neutrality. Keyed by the other civ's `CivId` in `Civilization.inter_civ_relations`. Both sides maintain their own entry; the records are independent and may diverge (a civ that was raided and a civ that raided have different perspectives on what happened).

Values accumulate from contact events — they are not assigned. `hostility` rises from raids and conflicts; `cooperation` rises from trade and cultural exchange. Both decay toward zero without contact, analogous to agent `Relationship` decay via `last_interaction_tick`.

| Field | Type | Notes |
|---|---|---|
| `first_contact_tick` | `u64` | Tick of first recorded interaction. |
| `last_contact_tick` | `u64` | For decay — accumulated values attenuate without contact. |
| `hostility` | `f32` | Accumulated from raids and conflicts. Decays over time. |
| `cooperation` | `f32` | Accumulated from trade and aid. Decays over time. |
| `cultural_exchange` | `f32` | Degree of concept and capability transmission that has occurred. Decays slowly. Used to weight diffusion probability when contact recurs. |
| `agreements` | `Vec<CivAgreement>` | Active and recently broken formal agreements. Empty until explicit diplomacy has occurred. |
| `contact_log` | `Vec<CivContactEntry>` | Contact events ordered by tick. Pruned by salience — low-salience entries fade; high-salience events persist indefinitely regardless of age. A devastating war from five centuries ago may still shape the relationship. Treaty formation and dissolution appear here as high-salience entries. |

### CivContactEntry

| Field | Type | Notes |
|---|---|---|
| `tick` | `u64` | When the contact occurred. |
| `contact_type` | `CivContactType` | What kind of interaction. |
| `initiator` | `CivId` | Which civ initiated. |
| `outcome` | `ContactOutcome` | `Success`, `Failure`, `Partial`. Drives how much aggregate fields shift. |
| `salience` | `f32` | Historical weight. High-salience entries decay slower and are pruned last. A founding war or a pivotal trade alliance stays in the log indefinitely; a routine border crossing fades quickly. |

### CivAgreement

Formal commitment between two civilizations. Whether a civ honors its obligations is determined by agent-level decisions — leader disposition, resource pressure, political stability — not by the agreement itself. The agreement is state; honoring it is emergent.

Broken agreements are retained with `AgreementStatus::Broken` rather than deleted. A violated pact is a high-salience historical fact that should raise hostility, reduce future cooperation potential, and mark the breaker in the contact log.

| Field | Type | Notes |
|---|---|---|
| `agreement_type` | `AgreementType` | What kind of commitment. |
| `formed_tick` | `u64` | When the agreement was established. |
| `status` | `AgreementStatus` | Current state. |

```
AgreementType:
    MutualDefense     // obligation to assist if the other is attacked
    NonAggression     // commitment not to raid or attack
    Tribute           // one-way resource flow; encodes asymmetric power
    TradeCompact      // formalized trade with mutual expectations
    Alliance          // broad cooperation; typically implies mutual defense
```

```
AgreementStatus:
    Active
    Broken { by: CivId, at_tick: u64 }   // who broke it and when; informs hostility and future trust
    Expired                               // lapsed without violation
```

### CivContactType

```
CivContactType:
    Raid              // one civ attacks another for resources or territory
    Trade             // resource exchange
    Migration         // population movement across civ boundaries
    Conflict          // organized, sustained violence; larger scale than a raid
    CulturalContact   // proximity-based concept or capability diffusion opportunity
```

### ContactOutcome

```
ContactOutcome:
    Success   // initiator achieved their goal
    Failure   // initiator was repelled or objective unmet
    Partial   // mixed result
```

---

## PhysicalWorld

| Field | Type | Notes |
|---|---|---|
| `tiles` | `BTreeMap<TileId, Tile>` | Map grid. |
| `climate` | `ClimateState` | Current climate parameters. Evolves each tick. |
| `disease_vectors` | `Vec<DiseaseVector>` | Active disease populations and spread state. |

### ClimateState

Global climate parameters. Evolves each tick via slow drift and occasional shocks.

```
ClimateState {
    temperature:   f32,   // global offset from baseline; affects terrain productivity and warmth need decay
    precipitation: f32,   // global moisture level; affects food and water availability
    volatility:    f32,   // rate of climate drift; high = faster change, more frequent shocks
}
```

### DiseaseVector

A single active disease population and its spread state. A world may have multiple active vectors simultaneously.

```
DiseaseVector {
    label:          String,
    virulence:      f32,         // transmission probability per contact per year
    lethality:      f32,         // death probability given infection
    immunity_decay: f32,         // rate at which acquired immunity fades, per year
    active_tiles:   Vec<TileId>, // tiles currently experiencing active spread
}
```

### Tile

| Field | Type | Notes |
|---|---|---|
| `id` | `TileId` | |
| `terrain` | `TerrainType` | `Grassland`, `Forest`, `Desert`, `Mountain`, `Wetland`, `Coast` |
| `elevation` | `f32` | Meters. |
| `resources` | `ResourceLevels` | Current levels: food, water, stone, wood, metal. All `f32`. |
| `resource_regeneration` | `RegenerationRates` | Per resource, per year. |
| `carrying_capacity` | `u32` | Max sustainable population given current resources and tech. |

#### TerrainType

```
TerrainType:
    Grassland   // open plains; high food, easy movement
    Forest      // dense woodland; moderate food, wood resource, slower movement
    Desert      // arid; low food and water, high warmth stress
    Mountain    // high elevation; low food, stone resource, very slow movement
    Wetland     // marshy; moderate food and water, disease risk elevated
    Coast       // shoreline; fishing access, trade route endpoint
```

#### ResourceLevels

Current extractable resource quantities on a tile. All `f32`, representing available units relative to a per-terrain baseline.

```
ResourceLevels {
    food:  f32,
    water: f32,
    stone: f32,
    wood:  f32,
    metal: f32,
}
```

#### RegenerationRates

Per-resource natural replenishment rate. Units per year. Affected by climate and capability (e.g., farming raises effective food regeneration).

```
RegenerationRates {
    food:  f32,
    water: f32,
    stone: f32,
    wood:  f32,
    metal: f32,
}
```

---

## CivilizationalMetrics

Lives on `Civilization.metrics`, not on `WorldState` directly. Every civilization has a `CivilizationalMetrics` struct; for distant civs most fields start at zero and are updated sparsely. For the focus civilization and groups in direct contact, all fields are tracked and updated each tick.

Continuous values that drive structural labels and threshold effects. Labels ("band", "chiefdom", "city-state") are derived analytically from these — they never drive simulation logic directly.

Each metric stores both its current value and a smoothed velocity (rate of change per year). Both are first-class fields, not derived on read.

| Field | Type | Notes |
|---|---|---|
| `social_scale` | `MetricValue` | Effective community size relative to Dunbar limits. |
| `administrative_complexity` | `MetricValue` | Layers of coordinating roles between individual and top. |
| `territorial_coherence` | `MetricValue` | Degree of bounded, defended territory. |
| `specialization_index` | `MetricValue` | Proportion of agents in non-subsistence roles. |
| `surplus_capacity` | `MetricValue` | Food and resource production beyond subsistence. |
| `ritual_specialization` | `MetricValue` | Proportion of meaning-activity concentrated in specialist agents. |
| `leadership_concentration` | `MetricValue` | Degree to which high-stakes decisions route through one agent. |
| `redistribution_centrality` | `MetricValue` | Fraction of resource flow passing through a single node. |

### MetricField

Identifies a specific metric for use in threshold comparisons (e.g., in `EmergenceConditions` and `Capability.metric_thresholds`). Covers both `CivilizationalMetrics` fields and the cohort-level summary fields available on all cohorts.

```
MetricField:
    // CivilizationalMetrics fields
    SocialScale
    AdministrativeComplexity
    TerritorialCoherence
    SpecializationIndex
    SurplusCapacity
    RitualSpecialization
    LeadershipConcentration
    RedistributionCentrality

    // Cohort summary fields (available for all cohorts including sparse ones)
    Cohesion
    ResourcePressure
    CapabilityLevel
```

### MetricValue

```
MetricValue {
    value:    f32,   // current position [0.0, 1.0]
    velocity: f32,   // smoothed rate of change, per year; negative = declining
}
```

### PopulationState

Used where population is a raw count rather than a normalized value, so `MetricValue` doesn't apply directly.

```
PopulationState {
    count:       u32,   // current headcount
    growth_rate: f32,   // smoothed fractional change per year; -0.03 = 3% annual decline
}
```

`growth_rate` is updated with the same EMA formula as `MetricValue.velocity`, using `(new_count - old_count) / (old_count * delta_t)` as the raw per-tick signal. Expressing it as a fraction keeps it comparable across groups of very different sizes.

Velocity is updated each tick using an exponential moving average to smooth out variable tick lengths:

```
velocity = α * (new_value - old_value) / delta_t + (1 - α) * old_velocity
```

`α` controls responsiveness — lower smooths more aggressively. A single-tick raw delta is too noisy when tick length varies by orders of magnitude.

### Why velocity is load-bearing

Position in metric space is ambiguous without direction. Two civilizations can share identical values while being in completely different situations:

- `specialization_index: 0.4` rising — a civilization consolidating, specialists being freed by surplus
- `specialization_index: 0.4` falling — a civilization fragmenting, specialists dying or returning to subsistence

The label "chiefdom" applies to both. Velocity tells you which story you're in.

**Collapse detection** — `resource_pressure` rising fast is a crisis regardless of its current value. Same value with negative or zero velocity is stable. Systems monitoring for collapse thresholds should check velocity, not just position.

**Amplifying feedbacks** — the most important dynamics are loops: specialization rising → surplus rising → specialization rises faster. These loops only appear in velocity data. Positive velocity that is itself accelerating (positive second derivative) is a strong consolidation signal; decelerating positive velocity suggests a ceiling is approaching.

**Threshold anticipation** — knowing a metric is approaching a structural threshold at current velocity lets the interestingness signal increase before the crossing, tightening ticks in anticipation rather than discovering the event after the fact.

**Articulation layer** — "Rowan's chiefdom is consolidating" vs "fragmenting" is determined by velocity sign, not position. The context builder reads velocity to generate accurate framing of the civilizational situation.

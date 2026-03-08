# State Model

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
| `agents` | `BTreeMap<AgentId, Agent>` | Active Tier 1 named agents. |
| `cohorts` | `BTreeMap<CohortId, Cohort>` | Tier 2 population cohorts. |
| `civilizations` | `BTreeMap<CivId, Civilization>` | Tier 3 statistical civilizations. |
| `world` | `PhysicalWorld` | Terrain, climate, resources, disease. |
| `archive` | `AgentArchive` | Immutable. Dead agents only. Nothing writes here after death. |
| `metrics` | `CivilizationalMetrics` | Continuous values driving structural labels. |

---

## WorldClock

| Field | Type | Notes |
|---|---|---|
| `year` | `i64` | Negative = BCE. Starts around -300,000. |
| `tick` | `u64` | Monotonically increasing tick counter. |
| `last_delta` | `f32` | Years elapsed in the most recent tick. |

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
| `tier` | `SimTier` | `Tier1` always for active named agents. |

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

#### BeliefEntry

| Field | Type | Notes |
|---|---|---|
| `concept_id` | `ConceptId` | What the belief is about — a deity, a causal rule, a taboo. |
| `strength` | `f32` | Conviction. Decays without reinforcement. |
| `generation_distance` | `u32` | Hops from original source. Increases with each transmission. Drives epistemic decay. |
| `knowledge_state` | `KnowledgeState` | `Ignorance`, `Misattribution`, `RuleWithoutUnderstanding`, `PartialUnderstanding`, `FullUnderstanding` |

### Knowledge

Distinct from belief. Empirically-grounded capability or causal understanding.

| Field | Type | Notes |
|---|---|---|
| `knowledge` | `BTreeMap<Domain, KnowledgeState>` | Per domain: medicine, agriculture, metallurgy, etc. |

---

## Cohort (Tier 2)

Population group simulated at aggregate level. Not individual agents.

| Field | Type | Notes |
|---|---|---|
| `id` | `CohortId` | |
| `label` | `String` | e.g., "Rowan's tribe — non-named members" |
| `population` | `u32` | Headcount. |
| `age_distribution` | `AgeDistribution` | Rough breakdown: children, adults, elders. |
| `need_satisfaction` | `NeedSatisfactionRates` | Aggregate satisfaction rates per need. |
| `trait_distribution` | `TraitDistribution` | Mean and variance per trait across cohort. |
| `belief_profile` | `BTreeMap<ConceptId, f32>` | Aggregate belief strength per concept. |
| `location` | `TileId` | Centroid or primary tile. |
| `affiliation` | `Option<AgentId>` | Named leader if one has emerged. |

---

## Civilization (Tier 3)

Statistical only. Produces boundary events; no internal reasoning.

| Field | Type | Notes |
|---|---|---|
| `id` | `CivId` | |
| `label` | `String` | |
| `population` | `u32` | |
| `cohesion` | `f32` | Internal unity. Low cohesion → fragmentation events. |
| `aggression` | `f32` | Disposition toward neighbors. |
| `resource_pressure` | `f32` | Food and land stress relative to population. |
| `capability_level` | `f32` | Rough proxy for military and economic capacity. |
| `dominant_belief_profile` | `BTreeMap<ConceptId, f32>` | Relevant when ideas spread across borders. |
| `location` | `RegionId` | Approximate geographic zone. |

---

## PhysicalWorld

| Field | Type | Notes |
|---|---|---|
| `tiles` | `BTreeMap<TileId, Tile>` | Map grid. |
| `climate` | `ClimateState` | Current climate parameters. Evolves each tick. |
| `disease_vectors` | `Vec<DiseaseVector>` | Active disease populations and spread state. |

### Tile

| Field | Type | Notes |
|---|---|---|
| `id` | `TileId` | |
| `terrain` | `TerrainType` | `Grassland`, `Forest`, `Desert`, `Mountain`, `Wetland`, `Coast` |
| `elevation` | `f32` | Meters. |
| `resources` | `ResourceLevels` | Current levels: food, water, stone, wood, metal. All `f32`. |
| `resource_regeneration` | `RegenerationRates` | Per resource, per year. |
| `carrying_capacity` | `u32` | Max sustainable population given current resources and tech. |

---

## CivilizationalMetrics

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

### MetricValue

```
MetricValue {
    value:    f32,   // current position [0.0, 1.0]
    velocity: f32,   // smoothed rate of change, per year; negative = declining
}
```

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

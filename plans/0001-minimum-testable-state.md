# Minimum Testable State — Implementation Plan

The goal of this document is a concrete, phased plan for reaching the smallest coherent
simulation that can be automatically tested. This is not a feature roadmap — it is the
foundation every later layer builds on.

## What "minimum testable" means

A deterministic tick pipeline with at least one named agent in a minimal physical world,
verifiable through `cargo test` with no UI and no LLM. Per design principle 8, the
articulation layer is deliberately excluded: removing it does not change how the simulation
runs, only how a player would understand it. That property makes it the right boundary for
a first testable slice.

## What is explicitly deferred

- Tauri / React UI shell
- Articulation layer (LLM, context builder, Ollama)
- Player intervention and action log
- Cultural transmission and concept registry
- Capability graph and discovery mechanics
- Multi-civilization contact and diplomacy
- Interestingness signal (replaced by a fixed stub `Δt`)
- Cohort aggregate mechanics beyond the sparse fields
- Religion, theology, and epistemology

All of these are additive. The type system and architectural invariants established in
Phases 1–9 below are what they build on.

---

## Project structure

A single Rust library crate at `sim/`. No binary target yet; the Tauri shell wraps
this crate later. The crate is pure library so all functionality is reachable from
`cargo test` without running a process.

```
sim/
  Cargo.toml
  src/
    lib.rs
    state/        — all data types (WorldState, Agent, Tile, ...)
    physical.rs   — physical update step
    needs.rs      — need decay
    action.rs     — action definitions, scoring, execution
    tick.rs       — full tick pipeline
    genesis.rs    — world generation from seed
```

Key dependencies:

| Crate | Purpose |
|---|---|
| `rand` + `rand_chacha` | Seeded `ChaChaRng`; `ChaChaRng` is serializable and deterministic across platforms |
| `serde` + `serde_json` | WorldState serialization for determinism verification |
| `ordered-float` | If `f32` needs to appear in `BTreeMap` keys; avoid where possible |

---

## Phase 1 — Project skeleton

Scaffold the crate. No simulation logic yet.

- `Cargo.toml` with correct dependencies
- Module structure as above, all files present with empty `pub mod` declarations
- A single smoke test: `assert!(true)`

**Why this matters:** establishes that the build works and modules compile before any
logic is written. Determinism and BTreeMap constraints are easier to enforce from the
start than to retrofit.

---

## Phase 2 — Core data model

Implement all types from `state-model.md` as Rust structs and enums. No logic — just
types that compile with correct field names, types, and visibility.

Types to implement (grouped by module):

**`state/ids.rs`**
- `AgentId`, `CohortId`, `CivId`, `ConceptId`, `CapabilityId`, `TileId`, `RegionId` — all `u64` newtypes
- `type Domain = CapabilityId`

**`state/agent.rs`**
- `Sex`, `Agent`, `AgentNeeds`, `AgentTraits`
- `Relationship`, `BondType`, `KinRelation`
- `MemoryEntry`, `EventRef`, `BeliefEntry`, `KnowledgeState`

**`state/world.rs`**
- `WorldState`, `WorldClock`
- `AgentArchive`

**`state/physical.rs`**
- `PhysicalWorld`, `Tile`, `TerrainType`
- `ResourceLevels`, `RegenerationRates`, `ClimateState`, `DiseaseVector`

**`state/cohort.rs`**
- `Cohort`, `AgeDistribution`, `NeedSatisfactionRates`, `TraitDistribution`
- `PopulationState`, `MetricValue`, `MetricField`

**`state/civ.rs`**
- `Civilization`, `CivilizationalMetrics`
- `CivRelation`, `CivContactEntry`, `CivAgreement`
- `CivContactType`, `ContactOutcome`, `AgreementType`, `AgreementStatus`

**`state/concepts.rs`** (empty registries for now)
- `Concept`, `ConceptType`, `UtilityModifier`, `TransmissionProfile`, `TransmissionMedium`, `EmergenceConditions`

**`state/capabilities.rs`** (empty registries for now)
- `Capability`, `DiscoveryMechanism`

**`state/action.rs`**
- `ActionTag` enum with the full list from the state model

**Invariant to enforce from the start:** every collection iterated during simulation
logic uses `BTreeMap` keyed by the appropriate ID type — never `HashMap`. This is
stated in `architecture.md` as the most common determinism footgun and must be
established at type definition time.

**Tests for Phase 2:**
- All types construct with `Default` or explicit construction without panicking
- BTreeMap fields accept the correct key and value types
- `AgentId(1) < AgentId(2)` — newtype ordering works correctly
- `KnowledgeState` variants have a defined ordering (for comparison in tests)

---

## Phase 3 — World genesis

A function `genesis(seed: u64) -> WorldState` that produces a minimal but valid starting
world deterministically from a seed.

**Minimal world spec:**
- 5×5 tile grid (25 tiles), IDs 1–25
- Terrain mix: central 3×3 Grassland cluster (high food, wood), border ring mixed Mountain/Forest
- One focus civilization with one cohort, one initial band of 6–10 named agents
- Agents start with all needs at 0.9 (well-fed, rested, safe — not perfect, not critical)
- Agents start with a flat trait distribution sampled from `ChaChaRng`
- Clock: `year = -300_000`, `tick = 0`, `last_delta = 0.0`
- `rng` initialized from seed via `ChaChaRng::seed_from_u64(seed)`

**Tests for Phase 3:**
- `genesis(42)` completes without panic
- `genesis(42)` called twice produces identical `WorldState` (same seed, same output)
- `genesis(42)` and `genesis(43)` produce different agent trait values (seeds diverge)
- All agent `cohort_id` values reference a valid `CohortId` in `world.cohorts`
- Agent count matches cohort `population.count`

---

## Phase 4 — Physical tick

The physical update step: resource regeneration using the correct exponential delta-time
formula. Climate modulation is stubbed (constant temperature and precipitation for now).

**Resource regeneration formula:**

```rust
// correct — exponential, matches architecture.md requirement
new_level = (current + regen_rate * delta_t).min(baseline_max)
```

Note: resource regeneration is additive (not multiplicative decay), but must still
accept a `delta_t` parameter so a 10-year tick regenerates 10× a 1-year tick. The
exponential formula from `architecture.md` applies to decay systems; regeneration
here is linear in `delta_t` but capped at a terrain-determined maximum. The key
constraint is that `delta_t` must be an explicit parameter — never assumed fixed.

**Disease vectors:** initialized as empty `Vec` and not yet simulated. Placeholder only.

**Tests for Phase 4:**
- Grassland tile with food at 0.5 regenerates correctly over `delta_t = 1.0`
- Grassland tile with food at 0.5 regenerates 10× as much over `delta_t = 10.0` (linearity check)
- Resource levels never exceed `baseline_max` for the terrain type
- Resource levels never go negative
- Physical update with no agents present completes without panic

---

## Phase 5 — Need decay

Exponential need decay for all agents. This is the step that makes needs actually matter
and drives action selection.

**Decay formula (from `architecture.md` — exact form required):**

```rust
new_value = current_value * (1.0 - rate).powf(delta_t)
```

Not:

```rust
new_value = current_value - (rate * delta_t)  // wrong — goes negative, not honest
```

Each need has a characteristic decay rate per year:

| Need | Decay rate/year | Notes |
|---|---|---|
| `food` | 0.90 | Critical within days; high rate |
| `water` | 0.98 | Critical within hours; very high rate |
| `sleep` | 0.85 | Critical within days |
| `shelter` | 0.15 | Slow decay; climate-modulated |
| `warmth` | 0.20 | Climate-dependent; stub for now |
| `safety` | 0.30 | Situational; spikes on threat events |
| `belonging` | 0.08 | Slow — weeks to months |
| `status` | 0.05 | Very slow; event-driven |
| `meaning` | 0.02 | Extremely slow — months to years |

These are initial values and will be tuned through play. They establish the right
qualitative ordering: water decays fastest, meaning decays slowest.

**Open question resolved here:** `open-questions.md` asks "what needs exist at
simulation start?" Recommendation: all nine needs above, initialized at 0.9 for all
agents at genesis. The decay rates above are the starting parameters.

**Tests for Phase 5:**
- `food` need at 1.0 with `delta_t = 1.0` decays to `1.0 * 0.1^1.0 = 0.1` (using rate 0.90)
- Verify exponential vs linear: `delta_t = 2.0` produces `current * 0.1^2 = 0.01`, not `current - 2*0.9`
- Need values are always in `[0.0, 1.0]` — never negative, never above 1.0
- `water` decays faster than `meaning` over the same `delta_t`
- Need decay with `delta_t = 0.0` produces no change (identity)

---

## Phase 6 — Action library and utility scoring

Minimal action set sufficient to demonstrate survival. Actions are not an FSM — agents
score all available actions and pick the highest, per `architecture.md`.

**Minimal action set for MVP:**

| ActionTag | Precondition | What it does |
|---|---|---|
| `Forage` | Tile food > 0.05 | Extracts food from tile; increases agent `food` need |
| `Rest` | None | Increases agent `sleep` need |
| `Socialize` | Another agent within range | Increases `belonging` need for both agents |

**Utility scoring:**

Each action returns a `f32` utility score given current agent state. Higher = more
likely to be chosen. The key design property: apparent behavioral state (e.g., "this
agent is hungry and foraging") emerges from scores, never from explicit state transitions.

Scoring sketch:

```rust
fn score_forage(agent: &Agent, tile: &Tile) -> f32 {
    // hunger drives forage utility
    let hunger = 1.0 - agent.needs.food;  // 0 = satisfied, 1 = critical
    let food_available = (tile.resources.food > 0.05) as u8 as f32;
    hunger * food_available * (1.0 + agent.traits.curious * 0.1)
}

fn score_rest(agent: &Agent) -> f32 {
    let exhaustion = 1.0 - agent.needs.sleep;
    exhaustion * 0.8  // rest is slightly less urgent than food at equal deprivation
}
```

Traits modulate scores through the weight curves — not through hard rules. `brave`
raises utility for actions with physical risk; `curious` gives a small bonus to
`Forage` and `Explore`. This is the mechanism by which traits shape emergent behavior.

**Action selection:**

```rust
fn select_action(agent: &Agent, world: &WorldState) -> ActionTag {
    let scores = available_actions(agent, world)
        .map(|action| (action, score(action, agent, world)));
    scores.max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0
}
```

Tie-breaking uses `ChaChaRng` with a deterministic draw. Agent ID ordering is not
sufficient for utility ties because two agents with identical state should not always
make the same choice — use the PRNG.

**Tests for Phase 6:**
- Agent with `food = 0.1` (near-critical) selects `Forage` over `Rest`
- Agent with `sleep = 0.05` (near-critical) and `food = 0.9` selects `Rest` over `Forage`
- Agent on a tile with `food = 0.0` does not have `Forage` in available actions
- Same agent state + same PRNG state → same action selected every time (determinism)
- Brave agent rates a risky forage higher than a timid agent given identical needs

---

## Phase 7 — Action execution and double-buffering

Apply action effects and enforce the double-buffer pattern.

**Effect of `Forage`:**

```rust
// amount extracted is bounded by tile availability and agent capacity
let extracted = (tile.resources.food * 0.2).min(1.0 - agent.needs.food);
tile_delta.food -= extracted;          // applied to start-of-tick copy
agent_delta.needs.food += extracted;   // clamped to 1.0 on apply
```

**Double-buffer protocol:**

```
1. Clone WorldState → read_state (tick-start snapshot)
2. For each agent (BTreeMap order — deterministic):
   a. Score and select action using read_state
   b. Compute delta against read_state
   c. Accumulate delta into pending_deltas
3. Apply all pending_deltas to WorldState atomically
```

This ensures that two agents foraging the same tile both see the same pre-tick food
level. Conflict resolution — when total extracted exceeds available — uses canonical
sort by `AgentId` (lower ID gets priority) applied after all deltas are collected.

**Tests for Phase 7:**
- Agent forages: tile food decreases, agent food increases by equal amount
- Two agents foraging the same tile: combined extraction does not exceed pre-tick food
- Agent with `food = 1.0` forages: need stays at 1.0 (no overshoot)
- Tile with `food = 0.0` after foraging: food does not go negative
- Double-buffer: agent action was selected against start-of-tick state, not mid-tick state
  (verify by checking that a second agent's action on the same resource was also computed
  from the start-of-tick value)

---

## Phase 8 — Full tick pipeline and agent lifecycle

Assemble all steps into a single `tick(world: &mut WorldState, delta_t: f32)` function.

**Pipeline order (from `architecture.md`):**

```
1. Physical update       — resource regeneration, climate step
2. Need decay            — exponential decay per agent per need over delta_t
3. Action selection      — utility scoring, pick highest (double-buffer read)
4. Action execution      — resource consumption, need restoration (double-buffer write)
5. Cultural transmission — stub / no-op for MVP
6. Threshold evaluation  — stub / no-op for MVP
7. Archival              — dead agents removed, written to AgentArchive
```

**Death condition:** any agent whose `food < 0.05` OR `water < 0.05` at the end of
step 7 is considered dead. This answers the open question about mortality thresholds.

**Archival:**

```rust
// Archival is the only step that modifies world.archive
let dead: Vec<AgentId> = world.agents.iter()
    .filter(|(_, a)| a.needs.food < 0.05 || a.needs.water < 0.05)
    .map(|(id, _)| *id)
    .collect();

for id in dead {
    let agent = world.agents.remove(&id).unwrap();
    world.archive.agents.insert(id, agent);  // sealed at death tick
}
```

**`AgentArchive` immutability:** enforced in tests by verifying that no function in
`tick.rs` has a write path into `world.archive` after initial insertion.

**Tests for Phase 8:**
- Well-supplied agent survives 50 ticks with food available
- Agent on tile with no food dies within expected number of ticks (calculable from decay rate)
- Dead agent appears in `archive`, not in `agents`
- Archive entry is not modified after death (re-run tick, verify archive entry unchanged)
- `world.clock.tick` increments by 1 each call
- `world.clock.year` increments by `delta_t` each call
- `world.clock.last_delta` stores the `delta_t` used

---

## Phase 9 — Determinism verification

The single most important architectural property. Verified through serialization comparison.

**Protocol:**

```rust
let world_a = genesis(42);
let world_b = genesis(42);

for _ in 0..100 {
    tick(&mut world_a, 1.0);
    tick(&mut world_b, 1.0);
}

let json_a = serde_json::to_string(&world_a).unwrap();
let json_b = serde_json::to_string(&world_b).unwrap();
assert_eq!(json_a, json_b);
```

If these diverge, there is a non-determinism bug. Likely causes, in order of probability:

1. A `HashMap` crept in somewhere — replace with `BTreeMap`
2. Floating-point operations are order-dependent — check loop bodies
3. A system call or timestamp was used — find and remove
4. The PRNG was advanced a different number of times — trace draw count

**Tests for Phase 9:**
- 100-tick run: two runs with seed 42 are byte-identical after serialization
- 100-tick run: seed 42 and seed 43 differ (confirms seeds actually diverge)
- 0-tick run: two identical worlds serialize identically (baseline for serialization correctness)
- 1-tick run, then serialize, then re-run from serialized state: identical to original (save/load round-trip)

---

## Completing the minimum testable state

After Phase 9, the following is true:

- Named agents are born (at genesis), live (forage/rest), and die (starvation)
- The tick pipeline runs all seven steps in the correct order
- All time-dependent systems use exponential delta-time formulas
- `BTreeMap` is used everywhere iteration order matters
- Double-buffering produces order-independent simultaneous updates
- Conflict resolution is canonical and documented
- `AgentArchive` is immutable after death
- A 100-tick run is byte-reproducible from the same seed
- Save/load round-trips correctly

This is the architectural foundation. Everything else is additive.

---

## Resolved open questions

From `open-questions.md`:

| Question | Resolution |
|---|---|
| What needs exist at simulation start? | All nine needs (food, water, sleep, shelter, warmth, safety, belonging, status, meaning), initialized at 0.9 |
| How do needs scale with delta-time? | Exponential formula: `current * (1.0 - rate).powf(delta_t)`. Rates specified in Phase 5 above. |
| What is the minimum viable capability graph at genesis? | Deferred. No capabilities at genesis — capability graph starts empty and grows via discovery. |

## Remaining open questions (deferred)

The following from `open-questions.md` are deliberately not resolved here — they require
play feedback, not upfront design:

- What does the interestingness signal weight each input? (Deferred until signal is implemented)
- What thresholds trigger LLM calls vs silent simulation? (Deferred until articulation layer)
- Win/lose or pure sandbox? (Deferred — no bearing on simulation mechanics)
- What happens when the player ignores prayers? (Deferred until player intervention)
- How is time compression communicated in the UI? (Deferred until UI)

## New questions surfaced by this plan

1. **What is the `delta_t` for the testing stub?** Recommendation: `1.0` year per tick
   as a constant for all MVP tests. The interestingness signal can vary it later; what
   matters now is that every system accepts it as a parameter and handles it correctly.

2. **What is the Grassland food baseline max?** Recommendation: `1.0` (normalized),
   regeneration rate `0.3` per year. Enough to sustain a band of 6–10 agents foraging
   at `20%` extraction per tick.

3. **What does the initial agent location assignment look like?** Recommendation:
   all agents start on the central Grassland tile. Spatial movement is deferred.

4. **Do agents move between tiles in MVP?** Recommendation: no. Agents are fixed to
   their starting tile for MVP. This defers pathfinding and spatial utility scoring
   while keeping the core loop testable.

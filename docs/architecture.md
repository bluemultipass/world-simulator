# Simulation Architecture

## The key insight
LLMs do not drive the simulation. Deterministic systems do. LLMs provide articulation on top — making the simulation legible to the player without being mechanically load-bearing.

## Three layers

**Physical layer — deterministic, causally indifferent**
The natural world operating by fixed rules regardless of agent belief or knowledge. Terrain, climate, biology, disease, celestial events. Distributions are used where appropriate but are themselves determined by world state and modified by human activity. This layer does not know agents exist.

**Agent layer — deterministic mechanics, emergent patterns**
Individual and collective human behavior. Need decay, utility scoring, trait expression, relationship dynamics, memory, cultural transmission, social organization, institutions, religion as lived practice. Deterministic at the mechanical level — emergent at the level of civilization. This is where everything interesting lives.

**Articulation layer — LLM-driven, async, non-blocking**
Gives voice to what agents experience and decide. Internal monologue, prayer generation, intervention response, event narration. A window into the agent layer, not a separate system. Mechanically inert — removing it would not change how the simulation runs, only how the player understands it. LLM calls are queued and resolved asynchronously, results inserted into the activity log when they return. A 150ms inference delay is invisible to the player.

---

## State Transition Model

World state is a vector of values — agent needs, relationships, resource levels, cultural memory weights, population counts, belief strengths, PRNG state. The state space is not a finite enumerable set. **The simulation is not a finite state machine.** Enumerating all possible states and transitions across hundreds of agents with continuous-valued attributes is not tractable and is not the right frame.

Transitions follow the form:

```
W(t + Δt) = F(W(t), Δt)
```

`F` is a pipeline of pure update functions applied in order each tick, each computing a partial delta to state. Closer in structure to a physics simulation or system of difference equations than to a state graph.

### The tick pipeline

1. **Physical update** — climate step, resource regeneration, disease spread
2. **Need decay** — exponential decay per agent per need over Δt
3. **Action selection** — utility scoring over available actions, pick highest
4. **Action execution** — resource consumption, memory formation, relationship updates
5. **Cultural transmission** — belief propagation with decay and mutation
6. **Threshold evaluation** — check continuous values against structural thresholds
7. **Archival** — dead agents written to history, removed from active state

Steps 1–5 are continuous delta computation. Step 6 is where qualitative structure changes.

### Agent action selection: utility AI, not FSM

Individual agents do not transition between explicit behavioral states via a graph. They score every available action against their current need state and context, then pick the highest. An agent does not transition `HUNGRY → FORAGING` via an explicit rule — it scores *forage* against *rest* against *socialize* given need levels and picks the winner. Apparent behavioral state emerges from utility scores. Deterministic given the same world state.

### Qualitative civilizational transitions are emergent threshold effects

Band becoming chiefdom, animism becoming institutionalized religion, kinship economy developing markets — none of these are explicitly modeled transitions. There is no edge in a graph labeled "band → chiefdom."

Instead, continuous metrics cross thresholds simultaneously:
- **Leadership concentration** — one agent consistently winning high-stakes utility decisions affecting others
- **Ritual specialization** — one agent spending disproportionate time on meaning-need activities
- **Resource redistribution** — consistent resource flow through a single node in the relationship graph

When enough of these cross simultaneously, an observer would call it a chiefdom. The simulation does not make that call — it updates continuous values. The label is analytical, not mechanical. The same pattern applies to religious institutions, markets, and political structures. This is design principle 3 (emergent over designed) expressed mechanically.

### Where discrete states do exist

The one genuinely FSM-like structure is **simulation tier** (Tier 1 / 2 / 3). Promotion and demotion are explicit events with documented rules. But this is a fidelity routing decision — it controls how much computational attention an agent receives, not what they do.

### Where complexity lives

In an FSM, complexity accumulates in the state graph — states and edges grow combinatorially with the number of phenomena modeled. In this sim, that complexity shifts to three places instead:

- **Action scoring curves** — what actions are available and how utility is computed for each given current need state and context. Getting the shape of these curves right is the core design work. A wrong curve is a single parameter change, not a graph restructure.
- **Threshold conditions** — which metrics, at which values, trigger structural changes. Wrong thresholds produce observable, diagnosable failure modes.
- **System interactions** — the feedback loops between dimensions. Disease + drought + low food utility → agents prioritize ritual over foraging → population collapses. No single rule produces this; the interaction does. This is where unpredictable but historically coherent behavior emerges.

### The action library

Actions are enumerated — states and transitions are not. The action library is finite at any moment but extensible and context-gated:

- A prehistoric band agent has ~8 available actions. A city-state agent has ~30.
- Actions gate on context: "pray to Asha" isn't scored until Asha exists in cultural memory; "legislate" isn't available until institutions exist.
- Adding a new action means writing one definition with its preconditions and utility contribution. Nothing else changes.

The action space expands as civilization develops, which is historically accurate, without requiring upfront anticipation of every possible human behavior.

### Civilizational state as metric space

The metrics that drive structural change — social_scale, administrative_complexity, territorial_coherence, specialization_index, and others — form a basis for a continuous space. Each civilization exists as a point in that space. "Band," "chiefdom," "city-state," "empire" are named regions defined by threshold hyperplanes. The label is a UI convenience; it never drives simulation logic.

**Trajectories matter as much as positions.** A civilization at the same metric values can be rising or collapsing — the velocity vector is as meaningful as the location. A city-state with declining specialization and rising territorial_coherence is becoming militarized. Same label, different trajectory, different story.

**The basis determines what's expressible.** A missing dimension means two civilizations that should be distinguishable look identical to the sim. Choosing the right metrics is the core design work — it is literally choosing what the simulation can perceive and model.

**Non-Western forms emerge without special casing.** Polynesian chiefdoms, Andean empires, stateless pastoral confederacies don't require their own types. If the metrics are well-chosen, they occupy distinct regions naturally. The model isn't culturally prescriptive; the labels carry cultural baggage, not the underlying space.

**The interesting dynamics are nonlinear.** Dimensions that amplify each other (specialization enables surplus, surplus enables specialization), dimensions that trade off under stress, collapse trajectories that look different from growth trajectories in the same region. These feedback loops produce the sim's character.

The dimensions are not orthogonal — they interact, which is the source of emergent behavior. But as a mental model: the sim constructs a basis and populates it with dynamics, not a graph of outcomes.

---

## World Structure

### Three-tier fidelity model

**Tier 1 — Individually tracked members**
Full simulation. Rich structs, memory, relationships, knowledge graphs, cultural memory, individual reasoning, articulation layer active. These are the individuals the player knows by name. They are members of a cohort — the cohort's aggregate stats include their states. The only thing that distinguishes them from unnamed members is that they are tracked individually. The pool is demand-driven up to a performance ceiling — not a fixed number. Early prehistoric band may have 8-12. A city-state may have 50-80. A nation at peak may have 150-200, most inactive at any given time.

Individual tracking is not tribal — it is relational. An agent is tracked individually because they matter to the story regardless of which group they belong to. Group affiliation is a property of the agent, not a criterion for individual tracking.

**Tier 2 — Full-pipeline cohorts**
The focus civilization and any group with meaningful relationship to it. A cohort represents the full population of a group — named agents included. All aggregate fields are populated and updated each tick: trait distributions, need satisfaction rates, belief profiles, capability profiles. Named members contribute their individual state to these aggregates each tick. Scales naturally as the focus group grows — a city of ten thousand does not need ten thousand Agent records; it needs a full-pipeline cohort and a named cast of ~30-80 individuals tracked at full fidelity.

**Tier 3 — Sparse-pipeline cohorts**
Foreign groups with no direct relationship to the focus civilization. **Same `Cohort` type as Tier 2 — different pipeline depth, not a different struct.** Each tick, only the sparse pipeline runs: population dynamics, resource pressure, aggression update, event threshold checks. Rich aggregate fields (trait distributions, need satisfaction, capability profiles) are empty. The belief profile contains only dominant beliefs sparse enough to drive border-crossing transmission.

This is a routing decision, not a type distinction. When a distant civilization makes contact, its cohort warms up: rich fields are populated by sampling from the summary stats already present, and it begins running the full pipeline. No migration, no struct change — just filling in previously empty fields and increasing pipeline depth.

High resource pressure plus high aggression produces raid events. Population growth past carrying capacity produces migration. Large capability differential plus aggression produces conquest attempts.

Every civilization — including the focus civilization — has both a `Civilization` record and a `Cohort`. The fidelity tier is a property of how much computation the cohort receives each tick, not of what type it is.

### Translation layer
Sparse-pipeline boundary events are converted into narrative events that named agents and full-pipeline cohorts can reason about. "Eastern group aggression crossed threshold and resource pressure is critical" becomes agents hearing rumors, scouts reporting strangers, fear spreading. The focus tribe responds with full fidelity to real external pressure.

---

## State Persistence and Replay

### What determinism actually means here

The simulation is deterministic given a fixed input stream: `seed + player action log`. Player interventions are external inputs — they change world state directly and causally affect everything that follows. A run with divine rain at tick 500 diverges permanently from the same seed with no intervention. "Same seed" alone does not reproduce a run the player has touched.

Correct formulation: **given the same seed and the same sequence of player actions, the simulation produces identical output every time.** The player action log is a required part of the replay input, not an optional addendum.

### Player action log

Every player intervention is appended to a persistent log: what action, at what tick, targeting what. This log, combined with the original seed, is sufficient to reproduce any past state by replaying from genesis. The log is append-only and never modified — it is the authoritative record of what the player did.

### Live world state — mutable in place

`WorldState` is mutated in place each tick. The double-buffer pattern (see below) ensures correctness within a tick — agents read state from tick start, updates apply atomically — but the previous tick's state is not preserved after the tick completes. Only the current state is live.

### Checkpoints

A checkpoint is a full serialization of `WorldState` at a specific tick. Checkpoints are taken automatically at notable events (major threshold crossings, player interventions, significant narrative moments) and on player request.

To reconstruct any past state: load the most recent checkpoint before that tick, then replay forward using the player action log from that checkpoint's tick to the target tick. Replay cost is bounded by the distance to the nearest checkpoint, not by total sim age.

Checkpoints absorb all prior player actions — they capture the world as it actually was, not as it would have been without intervention. This makes them more useful than pure seed-based replay for worlds with significant player history.

### `AgentArchive` — the only immutable object

Dead agents are written to `AgentArchive` and never modified again. Nothing else in the live simulation has a formal immutability guarantee. Live state evolves; the archive is fixed.

---

## Agent Layer Determinism

The agent layer is deterministic: given the same seed and player action log, the simulation produces identical output every time. This section documents the implementation constraints that make that true.

### Seeded PRNG — the foundation

All randomness flows through a single seeded `ChaChaRng`. The seed is part of world state and is serialized with it. Trait mutation at birth, utility tie-breaking, cultural transmission mutation, disease exposure, conflict lottery — all draw from this generator. It is passed explicitly to every system that needs it; no thread-local or global RNG state.

### Deterministic iteration order — the most common footgun

Rust's `HashMap` iterates in non-deterministic order by design. Iterating agents from a `HashMap` and applying updates sequentially produces different outcomes on different runs even with the same seed. **Rule: agent storage uses `BTreeMap` (keyed by agent ID) or a `Vec` with canonical sorting before iteration. `HashMap` is never used for any collection iterated during simulation logic.** This constraint must be established from day one — retrofitting it is painful.

### Double-buffering for simultaneous updates

When multiple agents act within the same tick, two approaches are possible:
- **Sequential**: process agent A, update shared state, then process agent B against updated state → outcome depends on iteration order
- **Double-buffer**: read all agents from state at tick start, compute all actions, then apply all updates atomically → outcome is order-independent

The double-buffer pattern is correct for this sim. Agent behavior within a tick responds to world state at the start of that tick, not to other agents' actions mid-tick. The tick rate is slow enough that the additional memory cost is irrelevant.

### Exponential delta-time for all decay and growth

Every time-dependent system uses exponential forms, not linear approximations. A need decaying at rate `r` per year over `Δt` years:

```
new_value = current_value * (1 - r).powf(Δt)   // correct
new_value = current_value - (r * Δt)            // wrong — goes negative, not mathematically honest
```

This applies to: need decay, cultural memory decay, relationship attenuation, population growth, belief drift. Linear forms are incorrect for variable tick lengths and must not appear in simulation logic.

### Generational transmission within a tick

If a tick represents 50 years and a generation is ~25 years, two full generational cycles occurred. Cultural transmission and memory decay must apply `floor(Δt / generation_length)` discrete steps, not a single aggregated decay. Each step applies its own transmission probability and mutation. The number of steps is deterministic given Δt and the generation length parameter.

### Conflict resolution for shared resources

When multiple agents contend for the same resource in the same tick (both read it as available via double-buffering), a canonical rule resolves priority: sort contenders by agent ID and process in order, or use the seeded PRNG for a lottery. The rule must be explicit and documented. Implicit ordering is where determinism breaks silently.

### Threshold-crossing events and variable tick length

A tick spanning 10 years may cross a threshold (epidemic onset, faith collapse, institutional formation) at year 7 — but this is only discoverable at tick completion. The resolution: **let the interestingness signal handle it**. As underlying conditions approach a threshold (disease load rising, meaning need critically unmet), the interestingness signal increases, ticks shorten, and threshold crossings naturally occur at finer granularity. This is not a workaround — it is the interestingness signal doing exactly what it is designed to do. No tick subdivision logic is needed.

---

## Named Agent Lifecycle

Agent lifecycle follows human lifecycle. Named agents are born, live, age, die. Whether they receive individual tracking changes based on relevance during their life. After death they become history.

**Promotion to individual tracking** — a cohort member gets an `Agent` record instantiated for them. The cohort's population count does not change — they were already in it. Their initial individual state is sampled from the cohort's trait and belief distributions. Conditions that trigger promotion:
- Role emergence — a new important social role forms and needs filling
- Threshold events — an agent does something significant enough to warrant individual tracking
- Player interest — player asks about or focuses on a specific person
- Proximity — close relationship with an existing named agent
- Relational significance — a foreign figure whose individual state materially affects the focus tribe's story

**Demotion from individual tracking** — the `Agent` record is dissolved. Their final state is folded back as adjustments to the cohort's aggregate distributions. The cohort's population count does not change. Used when a named agent becomes narratively irrelevant (not the same as death).

**Foreign named agents** fall into two cases:
- **Persistent** — figures who matter to the focus tribe's story over a long period. A rival king, a foreign prophet whose theology is spreading. Full Tier 1 treatment for the duration of their life.
- **Temporary** — figures relevant for a specific episode. A general during a war, a diplomat during negotiations. Promoted to Tier 1 for the episode, archived when it concludes. The Moses/Rameses dynamic: both are Tier 1 simultaneously during the episode, in direct relationship, individually simulatable.

**On death** — named agents are archived as immutable historical state. Nothing writes to the archive after death. The agent is no longer simulated. Their influence persists through what they did, what was transmitted, what institutions they built, what their descendants inherited — all handled by the existing cultural memory and transmission system. The archive is the player's omniscient window into what actually happened, not a source agents can access.

**Agents know only what cultural transmission delivered to them.** Their cultural memory of a historical figure is the only truth they have access to — stories, records, oral tradition, institutional interpretation, with all its decay and distortion. The articulation layer draws from agent knowledge state only. The gap between archived truth and living cultural memory is visible to the player. It is invisible to agents because they have no mechanism to access it. That gap is mythology.

**On resurrection** — if supported, the archived state is reactivated as the starting point. The agent returns as who they were, into a world that may have built theology around their death.

---

## Articulation Layer — Context Construction

The articulation layer never receives raw simulation values. A context builder translates agent state into natural language before each LLM call. The translation is deterministic and rule-based — not LLM-generated — and can be unit tested independently.

### Float → language translation

Continuous values map to natural language descriptions through range bands:

```
hunger: 0.85  →  "Rowan hasn't eaten in two days and is struggling to focus"
meaning: 0.12 →  "Rowan feels her prayers have gone unanswered; her faith is wavering"
```

Civilizational metrics work the same way. The LLM does not receive `social_scale: 0.73`. It receives what that value produces concretely: "Rowan lives in a chiefdom of ~200 people. Her leader Aldric distributes food from central stores. There are two ritual specialists." The translation layer owns this conversion; the LLM works from its output.

### Prompt structure

Each LLM call receives three components:

**Identity context** — who this agent is: traits, cultural background, belief profile, key relationships. Stable across calls for this agent. Cacheable.

**Situational snapshot** — current need levels in natural language, recent events the agent witnessed or participated in, current location and environment. Constructed fresh at call time from current agent state.

**Articulation task** — what to give voice to. Always narrow and already determined by simulation logic:
- "Rowan decided to forage rather than rest. Articulate her internal experience of that decision."
- "Rowan is praying. Her faith is wavering and she's hungry. Generate her prayer."
- "The player just sent rain. Rowan witnessed it. Articulate her reaction."

The LLM does not choose what happens. Utility AI already chose. The LLM gives voice to a decision already made by deterministic mechanics.

### Constraints

**The LLM cannot contradict simulation state.** If an agent is hungry, the articulation reflects hunger. If utility AI selected foraging, the internal monologue reflects that decision. The prompt is constructed so the LLM's degree of freedom is purely stylistic and emotional texture — not factual.

**Each call is a snapshot, not a session.** The LLM has no memory across calls. All context is injected fresh each time. This is what makes the async non-blocking architecture viable — calls are fully independent.

**Cultural voice lives in the translation layer.** An agent whose cultural memory contains strong flood mythology will have that context injected when articulating a river event. The LLM produces something that sounds like it comes from that culture because the prompt provides the relevant beliefs. The culture is encoded in the agent layer; the LLM speaks it.

**The context builder reads agent-accessible fields only — never the historical archive.** The gap between archived truth and living cultural memory is visible to the player and invisible to agents. This constraint enforces that gap mechanically.

---

## Time — Variable Tick Duration

Ticks are not fixed time units. Each tick represents a variable amount of world time determined by an interestingness signal. This is narrative time compression — the same pattern history uses when it records single days in exhaustive detail and summarizes centuries in a paragraph.

### The interestingness signal

A composite score derived from active world state. High score compresses time less; low score compresses more. Inputs include:

- Active conflicts or imminent threats
- Prayers pending player response
- Disease spreading through population
- Agent in critical need state
- Divine intervention just occurred and agents are responding
- Knowledge discovery in progress
- Demographic pressure approaching threshold
- Political instability or succession crisis

High interestingness → tick = minutes or hours. Low interestingness → tick = months or years. Transition is gradual, not binary.

### What this produces

- Prehistoric centuries with nothing remarkable compress to a few ticks
- A plague year receives full granular treatment
- A golden age of stability fast-forwards until something destabilizes it
- The player is never watching nothing happen

### The engineering constraint

Every time-dependent system must accept a delta-time parameter rather than assuming fixed decay rates. A tick representing a decade must propagate ten years of disease transmission, population growth, need decay, knowledge drift, and relationship change — not one day's worth. This is a foundational requirement that touches every system.

### Player experience of time compression

The UI must signal clearly when time is compressed vs granular. A decade passing in two ticks is disorienting without visual indication. The activity log naturally handles granular periods — events are narrated. Compressed periods need a different signal: a summary, a visual indicator, a change in log density.

### Sim character

This architecture produces a sim that feels closer to turn-based than streaming — each tick is a meaningful unit of narrative time, not a physics frame. This is appropriate. The player is observing and occasionally intervening in a story, not directing a real-time system. Dwarf Fortress operates similarly and it is the deepest sim ever made.

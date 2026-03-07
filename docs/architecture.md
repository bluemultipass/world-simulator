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

## World Structure

### Three-tier fidelity model

**Tier 1 — Named agents**
Full simulation. Rich structs, memory, relationships, knowledge graphs, cultural memory, individual reasoning, articulation layer active. These are the individuals the player knows by name. The pool is demand-driven up to a performance ceiling — not a fixed number. Early prehistoric band may have 8-12. A city-state may have 50-80. A nation at peak may have 150-200, most inactive at any given time.

Named agent pool membership is not tribal — it is relational. An agent enters the pool because they matter to the story regardless of which group they belong to. Tribal affiliation is a property of the agent, not a criterion for simulation tier.

**Tier 2 — Population simulation**
The broader focus civilization and any group with meaningful relationship to it. Not individual agents but not pure statistics. Modeled as cohorts with aggregate trait distributions, population dynamics, need satisfaction rates, belief profiles. Produces emergent events and realistic pressure on Tier 1 agents. Scales naturally as the focus group grows from tribe to city to nation — a city of ten thousand does not need ten thousand agents, it needs a Tier 2 cohort and a named cast of ~30-80 individuals who matter.

**Tier 3 — Statistical civilizations**
Foreign groups with no direct relationship to the focus civilization. Pure statistics evolving by simple deterministic rules. Produce boundary events only.

Each Tier 3 civilization tracks:
- **Population** — size and growth rate
- **Cohesion** — internal unity, resistance to fragmentation
- **Aggression** — disposition toward neighbors
- **Resource pressure** — food and land stress relative to population
- **Capability level** — rough proxy for military and economic capacity
- **Dominant belief profile** — relevant when ideas cross borders

High resource pressure plus high aggression produces raid events. Population growth past carrying capacity produces migration. Large capability differential plus aggression produces conquest attempts.

### Translation layer
Statistical boundary events are converted into narrative events that Tier 1 and Tier 2 agents can reason about. "Eastern group aggression crossed threshold and resource pressure is critical" becomes agents hearing rumors, scouts reporting strangers, fear spreading. The focus tribe responds with full fidelity to real external pressure.

---

## Named Agent Lifecycle

Agent lifecycle follows human lifecycle. Named agents are born, live, age, die. Simulation tier changes based on relevance during their life. After death they become history.

**Promotion to Tier 1** — conditions include:
- Role emergence — a new important social role forms and needs filling
- Threshold events — an agent does something significant enough to warrant individual tracking
- Player interest — player asks about or focuses on a specific person
- Proximity — close relationship with an existing named agent
- Relational significance — a foreign figure whose individual state materially affects the focus tribe's story

**Foreign named agents** fall into two cases:
- **Persistent** — figures who matter to the focus tribe's story over a long period. A rival king, a foreign prophet whose theology is spreading. Full Tier 1 treatment for the duration of their life.
- **Temporary** — figures relevant for a specific episode. A general during a war, a diplomat during negotiations. Promoted to Tier 1 for the episode, archived when it concludes. The Moses/Rameses dynamic: both are Tier 1 simultaneously during the episode, in direct relationship, individually simulatable.

**On death** — named agents are archived as immutable historical state. Nothing writes to the archive after death. The agent is no longer simulated. Their influence persists through what they did, what was transmitted, what institutions they built, what their descendants inherited — all handled by the existing cultural memory and transmission system. The archive is the player's omniscient window into what actually happened, not a source agents can access.

**Agents know only what cultural transmission delivered to them.** Their cultural memory of a historical figure is the only truth they have access to — stories, records, oral tradition, institutional interpretation, with all its decay and distortion. The articulation layer draws from agent knowledge state only. The gap between archived truth and living cultural memory is visible to the player. It is invisible to agents because they have no mechanism to access it. That gap is mythology.

**On resurrection** — if supported, the archived state is reactivated as the starting point. The agent returns as who they were, into a world that may have built theology around their death.

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

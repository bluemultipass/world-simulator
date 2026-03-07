# World Simulator

A civilizational-scale world simulator with ASCII art representation. The player is an omniscient god who observes and intervenes in the lives of simulated agents across prehistory to modernity. Agents have needs, beliefs, and internal reasoning. They can pray; the player can respond.

---

## Core Design Principles

**Player is God** — omniscient, can intervene at any time via natural language input

**Time is narrative, not fixed** — ticks represent variable amounts of world time driven by an interestingness signal. A plague year gets full granular treatment; prehistoric centuries with nothing remarkable compress to a few ticks. The player is never watching nothing happen.

---

## Documentation

| Document | Contents |
|---|---|
| [design-principles.md](docs/design-principles.md) | Guiding principles governing every design and implementation decision |
| [architecture.md](docs/architecture.md) | Three-layer simulation architecture, world fidelity tiers, agent lifecycle, variable tick/time system |
| [tech-stack.md](docs/tech-stack.md) | Language, frameworks, LLM runtime, hardware, UI layout |
| [physical-world.md](docs/physical-world.md) | Terrain, climate, biology, disease, astronomy, meteor strikes |
| [simulation-entities.md](docs/simulation-entities.md) | Needs, traits, relationships, memory, genesis capability set, knowledge & epistemology |
| [civilizational-scope.md](docs/civilizational-scope.md) | Capability graph, civilizational development, persistent identity |
| [religion.md](docs/religion.md) | Religion as emergent system, generational epistemology, god discovery, dynamic theology |
| [player-intervention.md](docs/player-intervention.md) | Intervention constraints, natural vs supernatural interventions, full catalogue with implementation tiers |
| [open-questions.md](docs/open-questions.md) | Unresolved design questions |

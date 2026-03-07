# Physical World

The bedrock layer. Operates independently of agent knowledge or belief. Agents are subject to it whether or not they understand it.

## Terrain & Environment
- **Terrain** — elevation, water bodies, soil fertility, resource deposits, climate zones
- **Climate** — temperature, rainfall, seasons, long-term drift and change
- **Flora** — plant life, food sources, raw materials, medicinal properties
- **Fauna** — prey species, predators, domesticable animals, disease vectors
- **Resources** — stone, metal, wood, fuel — discoverable and exhaustible

## Biology
- **Nutrition** — not just hunger but specific deficiencies (protein, calories, micronutrients) with distinct effects
- **Disease** — bacterial, viral, parasitic pathogens with transmission vectors, incubation periods, mortality curves
- **Epidemics** — triggered by population density thresholds, trade route contact, accumulating herd immunity
- **Genetics** — physical trait inheritance: disease resistance, strength, longevity, alongside behavioral traits
- **Injury and death** — acute vs chronic, treatable vs not given current knowledge state
- **Reproduction** — fertility rates, infant mortality, carrying capacity relative to food supply

## The key design principle
The physical layer does not care about agent interpretation. Pathogens kill regardless of whether agents attribute illness to divine punishment or bad air. The world is honest even when agents are wrong about it.

## Physical event distributions
Events are not randomly triggered — they emerge from underlying physical state. Distributions have base rates determined by world state and modifier layers shifted by human activity:

- Deforestation increases flood and erosion probability
- Dense settlement raises epidemic baseline transmission rate
- Overfarming degrades soil fertility distribution
- Resource extraction (equivalent to fracking) increases local seismic activity

Agents making locally rational decisions shift distributions without knowing it. Tragedy emerges from individually reasonable behavior. The physical layer does not warn them.

For genuinely stochastic micro-events — a specific lightning strike, an individual animal attack — probability distributions are appropriate. Civilizationally consequential events should emerge from state where possible.

---

## Astronomy

The sky is physical layer terrain. Celestial events — eclipses, planetary alignments, comet appearances, meteor strikes — are deterministic and scheduled from world generation based on the world's astronomy. They are fully predictable in principle.

Astronomical knowledge is a capability graph node: the ability to build accurate predictive models of celestial events. A civilization without it still experiences everything the sky produces — they simply cannot anticipate or explain it. Demystifying an eclipse is an epistemically significant threshold, stripping a major category of supernatural explanation from the world.

## Meteor strikes
Modeled at three scales:

- **Civilization-altering strikes** (Tunguska-scale to regional) — very low probability, seeded at world generation so they are deterministic within a run but varied across runs
- **Local strikes** — rare random events, physically modest, theologically significant out of proportion to their damage in pre-astronomical civilizations
- **Extinction-level strikes** — excluded entirely, outside human historical experience

Meteoric iron deserves explicit modeling. A strike near inhabited territory produces recoverable meteoric iron — a material that is workable, unusually hard, and fell from the sky. It is simultaneously a resource windfall, a potential early path toward iron-working in the capability graph, and a profound theological input. The same object does work on all three layers at once. Civilizations that encounter meteoric iron early develop along different capability paths than those that don't — contingent access to a rare physical resource shaping development organically.

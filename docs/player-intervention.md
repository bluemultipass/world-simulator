# Player Intervention

## Constraints and Consequences

### The core constraint: valid world state
Player interventions are mutations of the world state data structure. The constraint is not physics — it is representability. An intervention is valid if it produces a world state the simulation can continue from without special-casing or incoherence. Anything that can be written cleanly into the world state is allowed. Anything that requires the simulation to handle paradox or undefined state is not.

This means the player is not constrained by what the civilization currently knows or can do. Steel can be gifted to a prehistoric tribe. A river can be moved. A sea can be parted. These are all clean writes to the physical layer — the world state absorbs them and continues. The civilization's ability to exploit or understand the intervention is a separate question from whether the intervention is valid.

### Natural-layer interventions — no special handling required
These are direct mutations of existing world state structures and require no special simulation support:

- **Spawning objects** — add an item to a location (a steel sword, a food cache, a new material)
- **Moving terrain and water** — mutate physical layer values (redirect a river, part a sea, raise ground)
- **Healing or strengthening agents** — mutate biological state
- **Changing weather or climate** — mutate climate state
- **Creating animals** — spawn a non-agent entity
- **Gifting knowledge or gnosis** — write to an agent's knowledge/understanding state
- **Modifying resources** — add, remove, or relocate physical resources

These cover the vast majority of meaningful interventions. Apparent miraculousness to agents is determined by how far the intervention departs from what natural explanation can account for — not by any special simulation status.

### Supernatural interventions — require explicit first-class support
A small set of interventions cannot be handled as simple world state writes. They require deliberate design of underlying systems to support. These are the categorically miraculous events — the ones agents will recognize as impossible by any natural framework they possess.

**Resurrection** — requires the death system to be designed with reversibility in mind. Death likely triggers cleanup: removing the agent from active simulation, transferring knowledge to cultural record, updating surviving agents' relationships and memories. Resurrection means reversing some or all of that cleanup. If supported, it must be built into the death system from the start — retrofitting is expensive. This is an explicit design decision before implementation.

**De novo agent creation** — creating a fully formed sapient agent from nothing, outside the normal birth system. Probably implementable as an unusual invocation of the existing agent creation system, but requires that system to support arbitrary parameter initialization. A related question: can the player create an agent with no parents, no lineage, no cultural memory? What does the simulation do with that?

**Direct trait modification** — rewriting an agent's fundamental dispositional or moral traits, not just their knowledge or biological state. Potentially destabilizing to the simulation's behavioral consistency and worth treating as explicitly supernatural rather than a routine write.

### The theological distinction this produces
Natural-layer interventions are extraordinary but in principle explicable — agents can construct naturalistic accounts even if wrong. Explicitly supernatural interventions are categorically different. An agent returning from confirmed witnessed death has no naturalistic explanation available at any capability level. This maps cleanly onto what agents would recognize as genuinely miraculous versus merely remarkable, which matters for theology, prophet credibility, and the epistemic culture that develops.

### Apparent miraculousness decreases over time
An intervention that looks supernatural to a prehistoric tribe — curing disease, predicting an eclipse, producing fire — carries less theological weight in a civilization that understands those phenomena naturally. The player's options do not change. The gap between what the player does and what natural explanation can account for narrows as the capability graph develops.

### Intervention style is cumulative and irreversible
The theological culture that emerges is built from the full history of player behavior. Overt early intervention followed by long absence produces mythologization the player cannot undo. Subtle intervention produces epistemic cultures that may develop toward naturalism. The player cannot reset agent priors — only add new evidence to an existing interpretive history. Every intervention, including choosing not to intervene, is a permanent contribution to that history.

### Conflicting prayers
When agents pray for incompatible outcomes — victory in the same battle, survival through the same famine, favor over a rival — the player adjudicates. There is no system to resolve this automatically. The player's choice is a real decision with real consequences for who survives, who gains faith, and what theology develops in each affected group. This is the core gameplay tension.

### Open implementation decisions
These must be resolved before building the intervention and death systems:
- Is resurrection supported? If yes, death state must be designed as reversible from the start
- Is de novo agent creation supported? If yes, what are the constraints on initial state?
- Is direct trait modification supported? If yes, what are the behavioral consistency implications?

---

## Potential Supernatural Interventions

A curated list of supernatural intervention types worth considering, drawn from human religious and mythological traditions and game design. Each entry notes what world state support it requires and what emergent consequences it produces. None of these are committed — they are a design space to draw from.

### Life and Death

**Resurrection** — returning a dead agent to life. Theologically the most significant intervention in most human traditions. Requires reversible death state. The social consequences are enormous — witnesses, relationships, cultural memory of the death all exist and must now absorb an impossible fact. A resurrected agent's own psychological state is an open question.

**Healing** — curing disease, injury, or biological degradation. Implementable as a natural-layer write in most cases, but miraculous healing of conditions the civilization has no treatment for crosses into supernatural territory for agents. Relatively low implementation cost, high theological yield.

**Extending life** — slowing or halting aging for a specific agent. Produces a living continuity of memory across generations — an agent who was there at the founding and is still alive centuries later. Profound for cultural transmission and institutional authority. Requires aging to be a modifiable state.

**De novo agent creation** — creating a fully formed sapient agent. Mythologically common: golems, Adam and Eve, Athena from Zeus's head. Implementation requires birth system to support arbitrary initialization. What does a created agent believe about themselves? Do they have a soul, lineage, cultural memory?

**Taking an agent** — removing a living agent from the world entirely, the inverse of resurrection. Ascension, rapture, assumption. The agent is simply gone. Theologically interpreted as reward, punishment, or selection. Low implementation cost — essentially forced death without a body — but high narrative weight.

### Mind and Knowledge

**Gnosis** — direct instantiation of understanding already covered in the knowledge system. The supernatural version of education.

**Prophetic vision** — granting an agent foreknowledge of specific future events. Requires the simulation to generate a future state the agent receives as experience. Implementation complexity depends on how deterministic the sim's future is. The agent may or may not communicate it accurately, and other agents may or may not believe them.

**Divine madness** — inverting gnosis. Flooding an agent's mind with more than they can integrate, producing erratic behavior, fragmented insight, social disruption. Historically associated with oracles, shamans, and mystics. Could be accidental — an agent at the edge of the sim's knowledge representation.

**Memory alteration** — modifying what an agent remembers about a specific event. Subtle and potentially undetectable. Could be used to correct misattribution or introduce false belief. Significant implications for trust in the articulation layer's honesty.

**Tongue of angels** — granting an agent the ability to communicate with perfect clarity and persuasion, bypassing normal social trait constraints. Their words carry unusual weight. Implementation: temporary or permanent modifier on social influence calculations.

### Physical World

**Flood** — large-scale terrain and climate state mutation. Historically universal across mythologies. Requires water and terrain systems to support large-scale redistribution. Civilizationally catastrophic, theologically definitive.

**Plague** — directly seeding a pathogen into a population, bypassing normal transmission prerequisites. The inverse of healing at population scale. Requires disease system to accept external pathogen introduction.

**Manna / providence** — recurring resource provision without natural source. Food appearing, water flowing in desert. Low implementation cost — periodic resource spawning — but sustained intervention that agents will notice has a pattern and attribute accordingly.

**Pillar of fire / cloud** — a persistent environmental marker the player places in the world. Provides navigation, signals presence, marks territory as sacred. No mechanical effect beyond visibility but significant for agent behavior around the marked location.

**Tongues of fire / blessing of capability** — accelerating a specific capability graph node's discovery conditions without full gnosis. The conditions become more favorable rather than the knowledge being directly given. More subtle than gnosis, harder for agents to attribute, produces more organic-feeling development.

**Earthquake / volcanic eruption** — targeted seismic or geological intervention. Requires physical layer to support player-triggered events beyond normal distribution draws.

### Agents and Society

**Compelling** — temporarily overriding an agent's utility function to produce a specific action. The agent acts against their own weights. They will likely experience it as inexplicable compulsion and interpret it theologically. Significant implications for agent autonomy as a design value — use sparingly.

**Blinding / striking dumb** — temporarily or permanently removing a specific capability from an agent. Sight, speech, memory of a specific domain. The inverse of gnosis at the individual level.

**Mark of protection** — placing a persistent flag on an agent that modifies how other agents' threat assessment calculates against them. A marked agent is harder to harm, attack, or kill. Requires agent interaction system to check for marks.

**Curse** — the inverse of mark of protection, or a persistent negative modifier on an agent's or group's utility calculations. Can be specific (barrenness, bad luck, social rejection) or general. Historically associated with transgression of divine law — which in this sim means transgression of whatever the player has communicated they care about.

**Confusion** — introducing noise into a group's coordination mechanisms during conflict. Armies turn on each other, plans fail inexplicably. Implementation: temporary disruption to coalition and hierarchy mechanics.

### Covenant and Law

**Covenant** — a formal agreement between the player and a group, encoded in the simulation as a persistent relationship with defined terms. The group commits to specific behaviors; the player commits to specific interventions or protections. Requires a representation for formal agreements and their tracking.

**Divine law** — the player communicates a set of behavioral rules that agents interpret as binding from a supernatural source. Not enforced mechanically — agents follow or violate them based on their traits and circumstances, and the player decides whether to respond to violations. Produces the conditions for sin, guilt, and moral theology.

**Sign / covenant marker** — a recurring natural phenomenon the player designates as a symbol of relationship. A rainbow, a specific star, a seasonal event. No mechanical change to the phenomenon — but agents who know the covenant interpret it as divine communication. Pure articulation layer but with agent-layer behavioral consequences.

### Eschatological

**Apocalyptic vision** — granting widespread prophetic knowledge of a future catastrophe. Produces social reorganization, theological urgency, potential self-fulfilling or self-defeating prophecy dynamics depending on how agents respond.

**Judgment** — the player making a visible, unambiguous assessment of an agent or group's moral standing, expressed through intervention. Reward for the righteous, punishment for the wicked, in terms the civilization can observe. Requires the player to have communicated enough about their values that agents can interpret the judgment as such rather than arbitrary action.

---

## Implementation Priority Tiers

**Low cost, high yield** — worth building early:
Healing, manna/providence, plague, mark of protection, divine law, pillar of fire

**Medium cost, significant narrative value** — build when systems are ready:
Resurrection, prophetic vision, flood, covenant, curse, tongue of angels

**High cost, optional** — defer until core systems are stable:
De novo agent creation, memory alteration, compelling, eschatological interventions

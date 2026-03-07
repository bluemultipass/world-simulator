# Tech Stack

| Component | Choice | Reason |
|---|---|---|
| Language | Rust | Simulation logic performance, deterministic behavior |
| UI framework | Tauri + React | Native desktop window, full web ecosystem for rich UI, clean Rust/JS separation |
| LLM runtime | Ollama | Local model serving, REST API, easy Rust integration, model swapping without code changes |
| Model | Qwen2.5-7B Q4_K_M | ~4-5GB VRAM, capable narrative generation, conservative GPU usage |
| Hardware | RTX 3060 12GB | 7-8GB headroom after model load, async calls keep utilization low |

## Architecture

Simulation core runs in pure Rust, completely separate from UI. Tauri bridges the two via message passing. React frontend reads simulation state and renders it. Player input passes back through the bridge as events into the simulation.

```
Simulation core (pure Rust)
    ↓ state reads / events
Tauri bridge (message passing)
    ↓ renders
React frontend (native webview)
```

Tick rate is slow enough that bridge message passing overhead is negligible.

## Why Tauri over Dioxus
The UI requirements — capability graph visualization, world map with overlays, zoomable views, rich agent inspection panels — are solved problems in the web ecosystem. React Flow for the capability graph. Canvas or WebGL for the world map. Mature, battle-tested libraries available immediately. Dioxus has thinner ecosystem support for these and is niche enough that AI-assisted dev will hit hallucinated APIs regularly. React is one of the best-represented frameworks in LLM training data.

## Why not Ratatui
Ratatui is a TUI framework. Node graph visualization, map zoom, click-to-inspect panels, and rich intervention UI are not TUI-compatible in any satisfying way.

---

## UI Layout

- **Left panel** — ASCII world map, character-based, colored by entity/terrain type
- **Right panel** — scrolling activity log showing agent actions, locations, and internal monologue
- **Bottom panel** — agent needs table (Satiety, Energy, Fun, Social, Hygiene etc.)
- **Player input** — natural language intervention, fed into world state each tick

# Open Design Questions

- What does the interestingness signal weight each input? Needs tuning through play.
- What needs exist at simulation start and how do they scale with delta-time?
- What thresholds trigger narrative-worthy LLM calls vs silent simulation?
- What does a thriving vs failing civilization look like — is there a win/lose condition or is it pure sandbox?
- What happens when the player ignores prayers consistently — faith collapse, behavioral change, institutional drift?
- How is time compression communicated in the UI without breaking immersion?
- What is the minimum viable capability graph to start — what nodes exist at simulation genesis?
- What tile topology should the world use — square or hexagonal? Hex grids give uniform adjacency distance (6 equidistant neighbors vs. 4 cardinal + 4 diagonal at different distances for square), which matters for movement costs, spread mechanics, and perceived fairness. The state model is currently topology-agnostic (`Tile` has no coordinate fields; tiles are a flat `BTreeMap<TileId, Tile>`), so the decision is deferred until adjacency is needed. That decision point is whenever movement or spread is introduced.
- What is the right adjacency representation for large worlds? A naive `BTreeMap<TileId, Vec<TileId>>` stored in `PhysicalWorld` works for small grids but does not scale — a 1000×1000 hex world has 1 million tiles and 6 million neighbor entries to iterate during spread and movement steps. Alternatives worth evaluating: implicit neighbor computation from axial coordinates (no storage, O(1) lookup, requires coordinate fields on `Tile`); a compact CSR (compressed sparse row) adjacency structure (cache-friendly, read-only after genesis); or a chunked region map that limits spread propagation to local neighborhoods. The choice should be resolved before movement or disease spread is implemented.

// Physical update step: resource regeneration, climate step.

use crate::state::physical::PhysicalWorld;

/// Advance the physical world by `delta_t` years.
///
/// Resource regeneration is linear in `delta_t` and capped at each tile's terrain-determined
/// maximum (`tile.resource_max`). This matches the plan's formula:
///
///   new_level = (current + regen_rate * delta_t).min(baseline_max)
///
/// Climate modulation is stubbed: temperature and precipitation are held constant.
/// Disease vectors are initialized empty and not yet simulated.
pub fn physical_update(world: &mut PhysicalWorld, delta_t: f32) {
    for tile in world.tiles.values_mut() {
        let r = &tile.resource_regeneration;
        let max = &tile.resource_max;
        tile.resources.food = (tile.resources.food + r.food * delta_t)
            .min(max.food)
            .max(0.0);
        tile.resources.water = (tile.resources.water + r.water * delta_t)
            .min(max.water)
            .max(0.0);
        tile.resources.stone = (tile.resources.stone + r.stone * delta_t)
            .min(max.stone)
            .max(0.0);
        tile.resources.wood = (tile.resources.wood + r.wood * delta_t)
            .min(max.wood)
            .max(0.0);
        tile.resources.metal = (tile.resources.metal + r.metal * delta_t)
            .min(max.metal)
            .max(0.0);
    }
    // Climate step: stubbed — temperature and precipitation remain constant.
    // Disease vectors: not yet simulated.
}

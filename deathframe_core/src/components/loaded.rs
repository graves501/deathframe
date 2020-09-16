use super::component_prelude::*;

/// Entities which have `Loadable` and `Loaded` will be included in collision detection.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Loaded;

impl Component for Loaded {
    type Storage = NullStorage<Self>;
}

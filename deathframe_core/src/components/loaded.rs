use super::component_prelude::*;

/// Entities which have `Loadable` and `Loaded` will be included in collision detection.
#[derive(Default)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Loaded;

impl Component for Loaded {
    type Storage = NullStorage<Self>;
}

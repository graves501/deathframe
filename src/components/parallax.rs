use amethyst::ecs::world::Index;

use super::component_prelude::*;
use crate::geo::{Anchor, Vector};

/// A parallax background. This background has an offset and it can follow another entity,
/// similar to the camera. It is meant to be used with more parallax backgrounds.
/// The speed multiplier determines at what speed the image moves, relative to the following
/// entity. (Does not use velocity.)
pub struct Parallax {
    /// Positional offset.
    pub offset: Vector,
    /// Speed multipliers, values should be between 0.0 and 1.0.
    pub speed_mult: Vector,
    /// The ID of the entity to follow; doesn't make much sense without this.
    pub follow: Option<Index>,
    /// The anchor point of the following entity. Usually `Anchor::Middle`.
    pub follow_anchor: Anchor,
}

impl Parallax {
    pub fn new() -> ParallaxBuilder {
        ParallaxBuilder::default()
    }
}

impl Component for Parallax {
    type Storage = HashMapStorage<Self>;
}

impl Default for Parallax {
    fn default() -> Self {
        Self {
            offset:        (0.0, 0.0).into(),
            speed_mult:    (0.5, 0.5).into(),
            follow:        None,
            follow_anchor: Anchor::Middle,
        }
    }
}

/// A builder struct for `Parallax`.
pub struct ParallaxBuilder {
    offset:        Vector,
    speed_mult:    Vector,
    follow:        Option<Index>,
    follow_anchor: Anchor,
}

impl ParallaxBuilder {
    /// Set the `offset`.
    pub fn offset(mut self, offset: Vector) -> Self {
        self.offset = offset;
        self
    }

    /// Set the `speed_mult`.
    pub fn speed_mult(mut self, speed_mult: Vector) -> Self {
        self.speed_mult = speed_mult;
        self
    }

    /// Set the entity ID to follow.
    pub fn follow(mut self, entity_id: Index) -> Self {
        self.follow = Some(entity_id);
        self
    }

    /// Set the anchor point of the following entity.
    pub fn follow_anchor(mut self, anchor: Anchor) -> Self {
        self.follow_anchor = anchor;
        self
    }

    /// Build the `Parallax`.
    pub fn build(self) -> Parallax {
        Parallax {
            offset:        self.offset,
            speed_mult:    self.speed_mult,
            follow:        self.follow,
            follow_anchor: self.follow_anchor,
        }
    }
}

impl Default for ParallaxBuilder {
    fn default() -> Self {
        let Parallax {
            offset,
            speed_mult,
            follow,
            follow_anchor,
        } = Parallax::default();
        Self {
            offset,
            speed_mult,
            follow,
            follow_anchor,
        }
    }
}

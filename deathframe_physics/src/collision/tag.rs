/// This tag is used for collision checking and when moving with solids.
/// Implement this trait for your own type to use with `Collider` and `Collidable`.
/// This trait is automatically implemented for all types
/// implementing `PartialEq`. For those, the `collides_with` function
/// simply checks for equality between both types.
pub trait CollisionTag: Send + Sync + Clone {
    fn collides_with(&self, other: &Self) -> bool;
}

impl CollisionTag for () {
    fn collides_with(&self, _: &Self) -> bool {
        true
    }
}

// TODO
#[cfg(feature = "debug")]
impl<T> CollisionTag for T
where
    T: Send + Sync + Clone + PartialEq,
{
    fn collides_with(&self, other: &Self) -> bool {
        self == other
    }
}

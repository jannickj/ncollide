//!
//! Support mapping based Ball geometry.
//!

use nalgebra::vec::{AlgebraicVec, AlgebraicVecExt, ScalarSub, ScalarAdd};
use nalgebra::mat::Translation;
use geom::{HasMargin, Implicit};
use bounding_volume::{HasAABB, AABB};

/**
 * Implicit description of a ball geometry.
 * 
 *  - `N`: numeric type used for the ball radius.
 */
#[deriving(Eq, ToStr, Clone)]
pub struct Ball<N> {
    priv radius: N
}

impl<N> Ball<N> {
    /**
     * Creates a new ball from its radius and center.
     */
    #[inline]
    pub fn new(radius: N) -> Ball<N> {
        Ball { radius: radius }
    }
}

impl<N: Clone> Ball<N> {
    /**
     * The ball radius.
     */
    #[inline]
    pub fn radius(&self) -> N {
        self.radius.clone()
    }
}

impl<N: Clone> HasMargin<N> for Ball<N> {
    #[inline]
    fn margin(&self) -> N {
        self.radius.clone()
    }
}


impl<N: Algebraic + Clone, V: AlgebraicVec<N>, M: Translation<V>> Implicit<N, V, M> for Ball<N> {
    #[inline]
    fn support_point_without_margin(&self, m: &M, _: &V) -> V {
        m.translation()
    }
}

impl<N,
     V: AlgebraicVecExt<N> + Ord,
     M: Translation<V>>
HasAABB<N, V, M> for Ball<N> {
    #[inline]
    fn aabb(&self, m: &M) -> AABB<N, V> {
        AABB::new(m.translation().scalar_sub(&self.radius),
                  m.translation().scalar_add(&self.radius))
    }
}
//! Spatial partitioning tools.

pub use partitioning::dbvt::{DBVT, DBVTLeaf};
pub use partitioning::bvt::{BVT, kdtree_partitioner};
pub use partitioning::bvt_visitor::{BVTVisitor, RayInterferencesCollector, BoundingVolumeInterferencesCollector};

mod dbvt;
mod bvt;
mod bvt_visitor;

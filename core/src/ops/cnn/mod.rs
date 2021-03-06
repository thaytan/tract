mod patches;
pub mod conv;
pub mod pools;
mod avgpool;
mod maxpool;
mod padding;
mod patch_axis;

pub use self::pools::PoolSpec;
pub use self::avgpool::AvgPool;
pub use self::maxpool::MaxPool;
pub use self::padding::PaddingSpec;
pub use self::patches::{Patch, PatchSpec};
pub use self::patch_axis::PatchAxis;
pub use self::conv::{Conv, ConvUnary, KernelFormat};

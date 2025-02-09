mod auxv;
mod types;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub(crate) use auxv::linux_hwcap;
pub(crate) use auxv::page_size;
#[cfg(any(target_os = "android", target_os = "linux"))]
pub use types::{MembarrierCommand, RawCpuid};
#[cfg(not(target_os = "wasi"))]
pub use types::{RawGid, RawPid, RawUid, RawUname, EXIT_SIGNALED_SIGABRT};
pub use types::{EXIT_FAILURE, EXIT_SUCCESS};

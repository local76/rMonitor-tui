#[cfg(windows)]
mod monitor_win32_win;
#[cfg(windows)]
pub use monitor_win32_win::*;

#[cfg(not(windows))]
mod monitor_win32_mock;
#[cfg(not(windows))]
pub use monitor_win32_mock::*;

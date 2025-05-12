cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        pub mod linux;
        pub use linux::*;
    } else if #[cfg(target_vendor = "apple")] {
        pub mod apple;
        pub use apple::*;
    } else if #[cfg(target_os = "freebsd")] {
        pub mod freebsd;
        pub use freebsd::*;
    }
}

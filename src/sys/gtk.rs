cfg_if::cfg_if! {
    if #[cfg(feature = "gtk4_10")] {
        mod gtk4_10;
        pub(crate) use gtk4_10::*;
    } else {
        mod gtk4_legacy;
        pub(crate) use gtk4_legacy::*;
    }
}

#[macro_export]
#[cfg(target_os = "android")]
macro_rules! sdl2_log {
    (target: $target:expr, $($arg:tt)*) => (
        log!(target: $target, $crate::log::LogLevel::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        log!($crate::log::LogLevel::Debug, $($arg)*);
    )
}

#[macro_export]
#[cfg(target_os = "ios")]
macro_rules! sdl2_log {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        println!($($arg)*);
    })
}

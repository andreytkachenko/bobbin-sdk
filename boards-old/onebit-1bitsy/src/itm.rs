/// Macro for sending `print!`-formatted messages to the ITM (Instrumentation
/// Trace Macrocell).
#[macro_export]
macro_rules! iprint {
    ($s:expr) => {
        $crate::hal::itm::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::hal::itm::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages to the ITM, with a newline
#[macro_export]
macro_rules! iprintln {
    ($fmt:expr) => {
        iprint!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        iprint!(concat!($fmt, "\n"), $($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:literal) => {{
        info!($fmt,);
    }};

    ($fmt:literal $($args:tt)*) => {{
        eprintln!("[INFO]: {}", format_args!($fmt, $($args)*));
    }};
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal) => {{
        warn!($fmt,);
    }};

    ($fmt:literal, $($args:tt)*) => {{
        eprintln!("[WARN]: {}", format_args!($fmt, $($args)*));
    }};
}

#[macro_export]
macro_rules! err {
    ($fmt:literal) => {{
        err!($fmt,);
    }};

    ($fmt:literal, $($args:tt)*) => {{
        eprintln!("[ERR]: ({}:{}): {}", file!(), line!(), format_args!($fmt, $($args)*));
        std::process::exit(libc::EXIT_FAILURE);
    }};
}
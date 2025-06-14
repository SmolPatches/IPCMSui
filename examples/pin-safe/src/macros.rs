#[macro_export]
macro_rules! oinfo {
    // With target
    (target: $target:expr, $logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, $target, msg);
        log::info!(target: $target, "{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};

    // Without target
    ($logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, module_path!(), msg);
        log::info!("{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};
}

#[macro_export]
macro_rules! owarn {
    // With target
    (target: $target:expr, $logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, $target, msg);
        log::warn!(target: $target, "{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};

    // Without target
    ($logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, module_path!(), msg);
        log::warn!("{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};
}

#[macro_export]
macro_rules! oerror {
    // With target
    (target: $target:expr, $logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, $target, msg);
        log::error!(target: $target, "{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};

    // Without target
    ($logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, module_path!(), msg);
        log::error!("{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};
}
#[macro_export]
macro_rules! odebug {
    // With target
    (target: $target:expr, $logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, $target, msg);
        log::odebug!(target: $target, "{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};

    // Without target
    ($logger:expr, $($arg:tt)*) => {{
        use chrono::Utc;
        let msg = format!($($arg)*);
        let ts = Utc::now().to_rfc3339();
        let formatted = format!("[{} INFO  {}] {}", ts, module_path!(), msg);
        log::odebug!("{}", msg);
        $logger.write_message(&format!("{}\n", formatted));
    }};
}

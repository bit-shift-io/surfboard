/// Initialize the logger. This will use the default env_logger behavior,
/// setting the default log level to "info". The log format is also
/// customized to only include the log message, without the log level.
pub fn init_logger() {
    use std::io::Write;
    use env_logger::Builder;
    Builder::from_default_env()
        .format(|buf, record| {
            //let level = record.level();
            let message = record.args();
            writeln!(buf, "{}", message)
        })
        .init();
}

/// Initialize environment variables to default values.
pub fn init_env_var() {
    use std::env;
    env::set_var("RUST_LOG", "info");
    env::set_var("SLINT_BACKEND", "winit-skia");
}
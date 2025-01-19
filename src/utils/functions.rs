/// Initialize the logger. This will use the default env_logger behavior,
/// setting the default log level to "info". The log format is also
/// customized to only include the log message, without the log level.
pub fn init_logger() {
    use std::io::Write;
    use std::env;
    use env_logger::Builder;
    env::set_var("RUST_LOG", "info");
    Builder::from_default_env()
        .format(|buf, record| {
            //let level = record.level();
            let message = record.args();
            writeln!(buf, "{}", message)
        })
        .init();
}
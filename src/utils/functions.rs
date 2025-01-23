/// Initialize logger.
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
    //env::set_var("WGPU_BACKEND", "VULKAN");
}
pub fn init() {
    let env_var = env!("CARGO_PKG_NAME").to_uppercase().replace("-", "_");
    let env = env_logger::Env::default()
        .filter_or(format!("{}_LOG", env_var), "info")
        .write_style_or(format!("{}_LOG_STYLE", env_var), "always");

    env_logger::init_from_env(env);
}

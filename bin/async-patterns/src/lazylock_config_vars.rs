use std::{collections::HashMap, sync::LazyLock};

fn load_config() -> HashMap<String, String> {
    HashMap::from([(
        "DATABASE_URL".to_string(),
        "postgres://user:pass@localhost/db".to_string(),
    )])
}

static CONFIG: LazyLock<HashMap<String, String>> = LazyLock::new(load_config);

pub fn lazylock_config_vars() {
    // Initialization happens only once, on first access
    println!("Database URL: {}", CONFIG.get("DATABASE_URL").unwrap());
}

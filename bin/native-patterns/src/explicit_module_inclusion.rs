#[cfg(feature = "database")]
pub mod database {
    pub fn connect() {
        println!("Connecting to the database...");
    }
}

pub fn explicit_module_inclusion() {
    #[cfg(feature = "database")]
    {
        database::connect();
    }
}

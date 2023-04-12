
#[repr(C)]
pub struct LeafConfig {
    loglevel: LogLevel,
    number: u8,
}

#[repr(C)]
pub enum LogLevel {
    Verbose,
    Superverbose,
    Ultraverbose
}

#[no_mangle]
pub extern "C" fn hello(config: &LeafConfig) {
    println!("Hello from the leaf library, your number is {}", config.number);

    match config.loglevel{
        LogLevel::Verbose => println!("Verbose logging enabled!"),
        LogLevel::Superverbose => println!("Superverbose logging enabled!"),
        LogLevel::Ultraverbose => println!("Ultraverbose logging enabled!"),
    }
}

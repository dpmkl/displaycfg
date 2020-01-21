use displaycfg::Display;

#[derive(Display)]
pub struct Special {
    /// foobar
    test: u8,
}

/// Our configuration
#[derive(Display)]
pub struct Config {
    /// foo
    address: String,
    /// bar
    number: u16,
    /// baz
    debug: bool,
    /// wtf
    /// really
    test: Option<String>,
    /// special
    pub special: Special,
}

impl Config {
    pub fn new() -> Self {
        Self {
            address: "asdf".to_owned(),
            number: 123,
            debug: true,
            test: None,
            special: Special { test: 0 },
        }
    }
}

fn main() {
    let cfg = Config::new();
    println!("{}", Config::new());
    println!("{}", cfg.special);
}

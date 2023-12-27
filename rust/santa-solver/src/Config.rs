use std::collections::HashSet;

pub struct Config {
    pub enabled_methods: HashSet<String>,
    // todo: config for other algorithms (what puzzles to solve, etc.)
}

impl Config {
    pub fn new(enabled_methods: Vec<String>) -> Result<Config, &'static str> {
        let mut enabled_methods_set = HashSet::new();
        for method in enabled_methods {
            enabled_methods_set.insert(method);
        }
        Ok(Config {
            enabled_methods: enabled_methods_set,
        })
    }

    pub fn is_method_enabled(&self, method: &str) -> bool {
        self.enabled_methods.contains(method)
    } 
}



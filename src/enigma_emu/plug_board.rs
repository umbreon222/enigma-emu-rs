use std::collections::HashMap;

pub struct Plugboard {
    mapping: HashMap<char, char>,
}

impl Plugboard {
    pub fn new() -> Self {
        Plugboard {
            mapping: HashMap::new(),
        }
    }
    
    pub fn add_mapping(&mut self, a: char, b: char) {
        self.mapping.insert(a, b);
        self.mapping.insert(b, a);
    }
    
    pub fn map(&self, c: char) -> char {
        *self.mapping.get(&c).unwrap_or(&c)
    }
}

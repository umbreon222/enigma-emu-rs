pub struct Reflector {
    wiring: Vec<usize>,
}

impl Reflector {
    pub fn new(wiring: &str) -> Self {
        let mut mapping = vec![0; 26];
        for (i, c) in wiring.chars().enumerate() {
            mapping[i] = c as usize - 'A' as usize;
        }
        Reflector { wiring: mapping }
    }
    
    pub fn reflect(&self, c: usize) -> usize {
        self.wiring[c]
    }
}

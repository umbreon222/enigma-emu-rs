#[derive(Clone, Copy)]
pub struct Reflector {
    pub mappings: [usize; 26],
    pub position: usize
}

impl Reflector {
    pub fn new(mappings: [usize; 26]) -> Reflector {
        Reflector {
            mappings,
            position: 0
        }
    }

    pub fn feed_input(&self, index: usize) -> usize {
        let mut transformed_index: usize = index + self.mappings[index];

        // Normalize the index by wrapping around
        transformed_index %= 26;
        // DEBUG
        println!("Reflector transformed index to {}", transformed_index);
        let mapping = self.mappings[transformed_index];
        mapping
    }

    pub fn increment_position(&mut self) {
        self.position += 1;
        // Normalize the position by wrapping around
        self.position %= 26;
    }
}

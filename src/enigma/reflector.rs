#[derive(Clone, Copy)]
pub struct Reflector {
    pub mappings: [(usize, usize); 13],
    pub position: usize
}

impl Reflector {
    pub fn new(mappings: [(usize, usize); 13]) -> Reflector {
        Reflector {
            mappings,
            position: 0
        }
    }

    fn find_real_index(&self, index: usize) -> (usize, u8) {
        for i in 0..13 {
            let mapping = self.mappings[i];
            if index == mapping.0 {
                return (i, 0);
            }
            
            if index == mapping.1 {
                return (i, 1);
            }
        }

        panic!("Index \"{}\" is not mapped for reflector", index);
    }

    pub fn feed_input(&self, index: usize) -> usize {
        let real_index: (usize, u8) = self.find_real_index(index);
        let mut transformed_index: usize = real_index.0;
        transformed_index += self.position;

        // Normalize the index by wrapping around
        transformed_index %= 13;
        let mapping = self.mappings[transformed_index];
        if real_index.1 == 0 {
            mapping.0
        } else {
            mapping.1
        }
    }

    pub fn increment_position(&mut self) {
        self.position += 1;
        // Normalize the position by wrapping around
        self.position %= 13;
    }
}

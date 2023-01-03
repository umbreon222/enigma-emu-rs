#[derive(Clone, Copy)]
pub struct Rotor {
    pub is_reflector: bool,
    pub mappings: [(usize, usize); 26],
    pub notch: usize,
    pub position: usize
}

impl Rotor {
    pub fn new(is_reflector: bool, mappings: [(usize, usize); 26], notch: usize) -> Rotor {
        Rotor {
            is_reflector,
            mappings,
            notch: notch.into(),
            position: 0
        }
    }

    pub fn feed_input(&self, index: usize, forward: bool) -> usize {
        let mut transformed_index: usize = index.into();
        if forward {
            transformed_index += self.position;
        } else {
            // Rotate backwards
            transformed_index += 26 - self.position;
        }

        // Normalize the index by wrapping around
        transformed_index %= 26;
        let mapping = self.mappings[transformed_index];
        if forward {
            mapping.1
        } else {
            mapping.0
        }
    }

    pub fn increment_position(&mut self) -> bool {
        let should_increment_next_rotor_position = self.position == self.notch;
        self.position += 1;
        // Normalize the position by wrapping around
        self.position %= 26;
        should_increment_next_rotor_position
    }
}

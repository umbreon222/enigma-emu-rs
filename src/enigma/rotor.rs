#[derive(Clone, Copy)]
pub struct Rotor {
    pub mappings: [usize; 26],
    pub notch: usize,
    pub position: usize
}

impl Rotor {
    pub fn new(mappings: [usize; 26], notch: usize) -> Rotor {
        Rotor {
            mappings,
            notch: notch.into(),
            position: 0
        }
    }

    pub fn feed_input(&self, index: usize, forward: bool) -> usize {
        let mut transformed_index: usize = index;
        if forward {
            transformed_index += self.position;
        } else {
            // Rotate backwards
            transformed_index += 26 - self.position;
        }

        // Normalize the index by wrapping around
        transformed_index %= 26;
        if forward {
            self.mappings[transformed_index]
        } else {
            self.get_reversed_mappings()[transformed_index]
        }
    }

    pub fn get_reversed_mappings(&self) -> [usize; 26] {
        let mut reversed: [usize; 26] = self.mappings;
        for i in 0..26 {
            reversed[self.mappings[i]] = i;
        }
        
        reversed
    }

    pub fn increment_position(&mut self) -> bool {
        let should_increment_next_rotor_position = self.position == self.notch;
        self.position += 1;
        // Normalize the position by wrapping around
        self.position %= 26;
        should_increment_next_rotor_position
    }
}

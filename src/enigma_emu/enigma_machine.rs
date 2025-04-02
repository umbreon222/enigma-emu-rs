use super::{Rotor, Reflector, Plugboard};

pub struct EnigmaMachine {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    pub fn new(rotors: Vec<Rotor>, reflector: Reflector, plugboard: Plugboard) -> Self {
        EnigmaMachine {
            rotors,
            reflector,
            plugboard,
        }
    }
    
    pub fn set_rotor_positions(&mut self, positions: &str) {
        for (i, pos) in positions.chars().enumerate() {
            if i < self.rotors.len() {
                self.rotors[i].set_position(pos);
            }
        }
    }
    
    /// Step the rotors according to the Enigma machine mechanics
    fn step_rotors(&mut self) {
        // Check if middle rotor will cause a double step
        let middle_at_notch = if self.rotors.len() > 1 {
            self.rotors[1].position == self.rotors[1].notch
        } else {
            false
        };
        
        // Step the rotors (right to left)
        let mut step_next = true;
        for i in (0..self.rotors.len()).rev() {
            if step_next {
                step_next = self.rotors[i].step();
                
                // Handle double stepping of the middle rotor
                if i == 1 && middle_at_notch {
                    step_next = true;
                }
            }
        }
    }
    
    /// Encrypt or decrypt a single character
    fn process_char(&mut self, c: char) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }
        
        // Step the rotors before processing the character
        self.step_rotors();
        
        // Initial plugboard substitution
        let mut result = self.plugboard.map(c.to_ascii_uppercase());
        
        // Convert character to index (0-25)
        let mut index = result as usize - 'A' as usize;
        
        // Forward path through rotors (right to left)
        for rotor in self.rotors.iter().rev() {
            index = rotor.forward(index);
        }
        
        // Through the reflector
        index = self.reflector.reflect(index);
        
        // Return path through rotors (left to right)
        for rotor in &self.rotors {
            index = rotor.backward(index);
        }
        
        // Convert index back to character
        result = (index as u8 + b'A') as char;
        
        // Final plugboard substitution
        self.plugboard.map(result)
    }
    
    /// Encrypt or decrypt a message
    pub fn process_message(&mut self, message: &str) -> String {
        message.chars()
            .map(|c| self.process_char(c))
            .collect()
    }
}

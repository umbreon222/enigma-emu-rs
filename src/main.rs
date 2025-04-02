use std::collections::HashMap;

/// Represents a rotor in the Enigma machine
struct Rotor {
    // Wiring maps input positions to output positions
    wiring: Vec<usize>,
    // Reverse wiring for the signal's return path
    reverse_wiring: Vec<usize>,
    // Current position of the rotor (0-25)
    position: usize,
    // Position at which the next rotor will step
    notch: usize,
}

impl Rotor {
    fn new(wiring: &str, notch: char) -> Self {
        let mut forward = vec![0; 26];
        let mut backward = vec![0; 26];
        
        // Create forward wiring
        for (i, c) in wiring.chars().enumerate() {
            let out = c as usize - 'A' as usize;
            forward[i] = out;
        }
        
        // Create reverse wiring
        for (i, &out) in forward.iter().enumerate() {
            backward[out] = i;
        }
        
        Rotor {
            wiring: forward,
            reverse_wiring: backward,
            position: 0,
            notch: notch as usize - 'A' as usize,
        }
    }
    
    /// Process a character through the rotor in the forward direction
    fn forward(&self, c: usize) -> usize {
        let offset = (c + self.position) % 26;
        let result = self.wiring[offset];
        (result + 26 - self.position) % 26
    }
    
    /// Process a character through the rotor in the reverse direction
    fn backward(&self, c: usize) -> usize {
        let offset = (c + self.position) % 26;
        let result = self.reverse_wiring[offset];
        (result + 26 - self.position) % 26
    }
    
    /// Step the rotor by one position
    fn step(&mut self) -> bool {
        self.position = (self.position + 1) % 26;
        self.position == self.notch
    }
    
    /// Set the position of the rotor
    fn set_position(&mut self, position: char) {
        self.position = position as usize - 'A' as usize;
    }
}

/// Represents the reflector in the Enigma machine
struct Reflector {
    wiring: Vec<usize>,
}

impl Reflector {
    fn new(wiring: &str) -> Self {
        let mut mapping = vec![0; 26];
        for (i, c) in wiring.chars().enumerate() {
            mapping[i] = c as usize - 'A' as usize;
        }
        Reflector { wiring: mapping }
    }
    
    fn reflect(&self, c: usize) -> usize {
        self.wiring[c]
    }
}

/// Represents the plugboard in the Enigma machine
struct Plugboard {
    mapping: HashMap<char, char>,
}

impl Plugboard {
    fn new() -> Self {
        Plugboard {
            mapping: HashMap::new(),
        }
    }
    
    fn add_mapping(&mut self, a: char, b: char) {
        self.mapping.insert(a, b);
        self.mapping.insert(b, a);
    }
    
    fn map(&self, c: char) -> char {
        *self.mapping.get(&c).unwrap_or(&c)
    }
}

/// The complete Enigma machine
struct EnigmaMachine {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl EnigmaMachine {
    fn new(rotors: Vec<Rotor>, reflector: Reflector, plugboard: Plugboard) -> Self {
        EnigmaMachine {
            rotors,
            reflector,
            plugboard,
        }
    }
    
    fn set_rotor_positions(&mut self, positions: &str) {
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
    fn process_message(&mut self, message: &str) -> String {
        message.chars()
            .map(|c| self.process_char(c))
            .collect()
    }
}

fn main() {
    // Define historical rotor wirings
    let rotor_i = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q'); // Notch at Q
    let rotor_ii = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E'); // Notch at E
    let rotor_iii = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V'); // Notch at V
    
    // Define reflector B wirings
    let reflector_b = Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
    
    // Create a plugboard with some connections
    let mut plugboard = Plugboard::new();
    plugboard.add_mapping('A', 'M');
    plugboard.add_mapping('F', 'I');
    plugboard.add_mapping('N', 'V');
    plugboard.add_mapping('P', 'S');
    plugboard.add_mapping('T', 'U');
    plugboard.add_mapping('W', 'Z');
    
    // Create an Enigma machine with 3 rotors
    let mut enigma = EnigmaMachine::new(
        vec![rotor_i, rotor_ii, rotor_iii],
        reflector_b,
        plugboard
    );
    
    // Set initial rotor positions
    enigma.set_rotor_positions("AAA");
    
    // Example message
    let message = "HELLO WORLD";
    println!("Original message: {}", message);
    
    let encrypted = enigma.process_message(message);
    println!("Encrypted message: {}", encrypted);
    
    // Reset rotor positions to decrypt
    enigma.set_rotor_positions("AAA");
    let decrypted = enigma.process_message(&encrypted);
    println!("Decrypted message: {}", decrypted);
}

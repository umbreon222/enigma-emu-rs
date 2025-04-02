pub struct Rotor {
    // Wiring maps input positions to output positions
    wiring: Vec<usize>,
    // Reverse wiring for the signal's return path
    reverse_wiring: Vec<usize>,
    // Current position of the rotor (0-25)
    pub position: usize,
    // Position at which the next rotor will step
    pub notch: usize,
}

impl Rotor {
    pub fn new(wiring: &str, notch: char) -> Self {
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
    pub fn forward(&self, c: usize) -> usize {
        let offset = (c + self.position) % 26;
        let result = self.wiring[offset];
        (result + 26 - self.position) % 26
    }
    
    /// Process a character through the rotor in the reverse direction
    pub fn backward(&self, c: usize) -> usize {
        let offset = (c + self.position) % 26;
        let result = self.reverse_wiring[offset];
        (result + 26 - self.position) % 26
    }
    
    /// Step the rotor by one position
    pub fn step(&mut self) -> bool {
        self.position = (self.position + 1) % 26;
        self.position == self.notch
    }
    
    /// Set the position of the rotor
    pub fn set_position(&mut self, position: char) {
        self.position = position as usize - 'A' as usize;
    }
}

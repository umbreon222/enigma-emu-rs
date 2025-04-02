mod enigma_emu;

use enigma_emu::{
    Rotor,
    Reflector,
    Plugboard,
    EnigmaMachine
};

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

pub mod enigma;

pub use enigma::Enigma;

const PLUG_BOARD_MAPPINGS: [usize; 26] = [
    19,
    9,
    13,
    16,
    10,
    20,
    2,
    4,
    22,
    6,
    25,
    3,
    21,
    17,
    8,
    23,
    11,
    12,
    5,
    14,
    1,
    15,
    18,
    0,
    7,
    24
];

fn main() {
    let mut enigma_machine_simulator = Enigma::new([0, 1, 2], 0, PLUG_BOARD_MAPPINGS);
    enigma_machine_simulator.set_initial_rotor_positions([0, 0, 0, 0]);
    println!("Running simulation");
    /* DEBUG println!("Inputting \"helloworld\" to Enigma");
    let output = enigma_machine_simulator.run_simulation("helloworld"); */
    // DEBUG
    let output = enigma_machine_simulator.run_simulation("a");
    println!("Enigma output: \"{}\"", output);
    println!("Resetting Enigma rotor positions");
    enigma_machine_simulator.set_initial_rotor_positions([0, 0, 0, 0]);
    // DEBUG
    println!();
    println!("Feeding output back into Enigma");
    println!("Enigma output: \"{}\"", enigma_machine_simulator.run_simulation(&output));
}

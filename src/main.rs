pub mod enigma;

pub use enigma::Enigma;

const PLUG_BOARD_MAPPINGS: [(usize, usize); 26] = [
    (19, 7),
    (9, 24),
    (13, 9),
    (16, 19),
    (10, 21),
    (20, 3),
    (2, 25),
    (4, 13),
    (22, 17),
    (6, 23),
    (25, 10),
    (3, 20),
    (21, 15),
    (17, 4),
    (8, 14),
    (23, 8),
    (11, 22),
    (12, 16),
    (5, 11),
    (14, 2),
    (1, 5),
    (15, 18),
    (18, 12),
    (0, 0),
    (7, 6),
    (24, 1)
];

fn main() {
    let mut enigma_machine_simulator = Enigma::new([0, 1, 2], 0, PLUG_BOARD_MAPPINGS);
    enigma_machine_simulator.set_initial_rotor_positions([1, 0, 0, 0]);
    println!("{}", enigma_machine_simulator.encrypt("helloworld"));
}

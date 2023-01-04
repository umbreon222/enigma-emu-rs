pub use crate::enigma::Rotor;

const LOWER_CASE_LETTERS: [char; 26] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j', 
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't', 
    'u',
    'v',
    'w',
    'x',
    'y', 
    'z',
];

const AVAILABLE_ROTORS: [Rotor; 3] = [
    Rotor {
        is_reflector: false,
        mappings: [
            (9, 15),
            (2, 18),
            (11, 6),
            (13, 20),
            (4, 22),
            (10, 16),
            (25, 17),
            (12, 1),
            (1, 5),
            (22, 8),
            (6, 13),
            (20, 21),
            (7, 19),
            (24, 9),
            (17, 3),
            (18, 0),
            (3, 24),
            (8, 12),
            (19, 23),
            (16, 10),
            (23, 14),
            (15, 11),
            (14, 7),
            (5, 2),
            (0, 25),
            (21, 4)
        ],
        notch: 6,
        position: 0
    },
    Rotor {
        is_reflector: false,
        mappings: [
            (0, 25),
            (15, 8),
            (5, 12),
            (12, 13),
            (3, 10),
            (2, 18),
            (18, 15),
            (16, 11),
            (20, 4),
            (8, 22),
            (24, 19),
            (4, 16),
            (19, 20),
            (14, 24),
            (23, 2),
            (9, 7),
            (25, 9),
            (13, 1),
            (1, 21),
            (7, 5),
            (6, 3),
            (22, 0),
            (21, 23),
            (11, 17),
            (10, 14),
            (17, 6)
        ],
        notch: 1,
        position: 0
    },
    Rotor {
        is_reflector: false,
        mappings: [
            (16, 8),
            (0, 20),
            (20, 6),
            (18, 16),
            (24, 17),
            (4, 7),
            (1, 18),
            (9, 25),
            (10, 4),
            (12, 19),
            (21, 5),
            (25, 14),
            (13, 2),
            (7, 9),
            (5, 21),
            (15, 1),
            (3, 0),
            (23, 3),
            (11, 10),
            (22, 23),
            (17, 12),
            (2, 24),
            (19, 13),
            (8, 11),
            (14, 22),
            (6, 15)
        ],
        notch: 0,
        position: 0
    }
];

 const AVAILABLE_REFLECTORS: [Rotor; 1] = [
    Rotor {
        is_reflector: true,
        mappings: [
            (6, 3),
            (4, 17),
            (21, 21),
            (2, 14),
            (23, 4),
            (0, 0),
            (3, 20),
            (5, 2),
            (24, 18),
            (18, 15),
            (8, 23),
            (22, 25),
            (25, 12),
            (13, 5),
            (11, 16),
            (16, 1),
            (14, 13),
            (17, 24),
            (12, 8),
            (7, 6),
            (20, 22),
            (19, 7),
            (9, 11),
            (1, 10),
            (10, 9),
            (15, 19)
        ],
        notch: 25,
        position: 0
    }
];

pub struct Enigma {
    rotors: [Rotor; 4], // The fourth "Rotor" is the reflector
    plug_board: Rotor
}

impl Enigma {
    pub fn new(rotor_numbers: [usize; 3], reflector_number: usize, plug_board_mappings: [(usize, usize); 26]) -> Enigma {

        let rotors = [
            AVAILABLE_ROTORS[rotor_numbers[0]],
            AVAILABLE_ROTORS[rotor_numbers[1]],
            AVAILABLE_ROTORS[rotor_numbers[2]],
            AVAILABLE_REFLECTORS[reflector_number]
        ];

        let plug_board = Rotor::new(false, plug_board_mappings, 25);

        Enigma {
            rotors,
            plug_board
        }
    }

    pub fn set_initial_rotor_positions(&mut self, initial_rotor_positions: [usize; 4]) {
        for i in 0..4 {
            self.rotors[i].position = initial_rotor_positions[i];
        }
    }

    pub fn run_simulation(&mut self, input_string: &str) -> String {
        let input = Enigma::string_to_letter_indices(input_string);
        let transformed_input = self.transform_input(input);
        let output = Enigma::letter_indices_to_string(transformed_input);

        output
    }

    fn transform_input(&mut self, input: Vec<usize>) -> Vec<usize> {
        let mut output: Vec<usize> = vec![];
        for alphabet_index in input {
            let mut index = alphabet_index;
            // Feed forward through rotors
            index = self.rotors[0].feed_input(index, true);
            index = self.rotors[1].feed_input(index, true);
            index = self.rotors[2].feed_input(index, true);
            // Feed forward through reflector
            index = self.rotors[3].feed_input(index, true);
            // Feed backwards through rotors
            index = self.rotors[2].feed_input(index, false);
            index = self.rotors[1].feed_input(index, false);
            index = self.rotors[0].feed_input(index, false);
            output.push(index);

            // Simulator rotor rotations
            let mut should_rotate_next_rotor = self.rotors[0].increment_position();
            if should_rotate_next_rotor {
                should_rotate_next_rotor = self.rotors[1].increment_position();
            }

            if should_rotate_next_rotor {
                should_rotate_next_rotor = self.rotors[2].increment_position();
            }

            if should_rotate_next_rotor {
                self.rotors[3].increment_position();
            }
        }

        output
    }

    fn string_to_letter_indices(input_string: &str) -> Vec<usize> {
        let mut letter_indices: Vec<usize> = vec![];
        for letter in input_string.chars() {
            for i in 0..26 {
                if letter == LOWER_CASE_LETTERS[i] {
                    letter_indices.push(i);
                }
            }
        }

        letter_indices
    }

    fn letter_indices_to_string(letter_indices: Vec<usize>) -> String {
        let mut result = String::new();
        for letter_index in letter_indices {
            result.push(LOWER_CASE_LETTERS[letter_index]);
        }

        result
    }
}

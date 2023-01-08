pub use crate::enigma::{ Rotor, Reflector };

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
        mappings: [
            9,
            2,
            11,
            13,
            4,
            10,
            25,
            12,
            1,
            22,
            6,
            20,
            7,
            24,
            17,
            18,
            3,
            8,
            19,
            16,
            23,
            15,
            14,
            5,
            0,
            21
        ],
        notch: 6,
        position: 0
    },
    Rotor {
        mappings: [
            0,
            15,
            5,
            12,
            3,
            2,
            18,
            16,
            20,
            8,
            24,
            4,
            19,
            14,
            23,
            9,
            25,
            13,
            1,
            7,
            6,
            22,
            21,
            11,
            10,
            17
        ],
        notch: 1,
        position: 0
    },
    Rotor {
        mappings: [
            16,
            0,
            20,
            18,
            24,
            4,
            1,
            9,
            10,
            12,
            21,
            25,
            13,
            7,
            5,
            15,
            3,
            23,
            11,
            22,
            17,
            2,
            19,
            8,
            14,
            6
        ],
        notch: 0,
        position: 0
    }
];

 const AVAILABLE_REFLECTORS: [Reflector; 1] = [
    Reflector {
        mappings: [
            (0, 1),
            (2, 3),
            (4, 5),
            (6, 7),
            (8, 9),
            (10, 11),
            (12, 13),
            (14, 15),
            (16, 17),
            (18, 19),
            (20, 21),
            (22, 23),
            (24, 25)
        ]
    }
];

pub struct Enigma {
    rotors: [Rotor; 3],
    reflector: Reflector,
    plug_board: Rotor
}

impl Enigma {
    pub fn new(rotor_numbers: [usize; 3], reflector_number: usize, plug_board_mappings: [usize; 26]) -> Enigma {

        let rotors = [
            AVAILABLE_ROTORS[rotor_numbers[0]],
            AVAILABLE_ROTORS[rotor_numbers[1]],
            AVAILABLE_ROTORS[rotor_numbers[2]],
        ];

        let reflector = AVAILABLE_REFLECTORS[reflector_number];
        let plug_board = Rotor::new(plug_board_mappings, 25);

        Enigma {
            rotors,
            reflector,
            plug_board
        }
    }

    pub fn set_initial_rotor_positions(&mut self, initial_rotor_positions: [usize; 3]) {
        for i in 0..3 {
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
            println!("Beginning transform on \"{}\"", index);
            // Feed forward through rotors
            index = self.rotors[0].feed_input(index, true);
            println!("Output from rotor 1 \"{}\"", index);
            index = self.rotors[1].feed_input(index, true);
            index = self.rotors[2].feed_input(index, true);
            // Feed forward through reflector
            index = self.reflector.feed_input(index);
            println!("Output from reflector \"{}\"", index);
            // Feed backwards through rotors
            index = self.rotors[2].feed_input(index, false);
            index = self.rotors[1].feed_input(index, false);
            index = self.rotors[0].feed_input(index, false);
            println!("Output from rotor 1 \"{}\"", index);
            output.push(index);

            // Simulator rotor rotations
            let mut should_rotate_next_rotor = self.rotors[0].increment_position();
            if should_rotate_next_rotor {
                should_rotate_next_rotor = self.rotors[1].increment_position();
            }

            if should_rotate_next_rotor {
                self.rotors[2].increment_position();
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

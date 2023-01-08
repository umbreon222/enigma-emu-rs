#[derive(Clone, Copy)]
pub struct Reflector {
    pub mappings: [(usize, usize); 13]
}

impl Reflector {
    pub fn new(mappings: [(usize, usize); 13]) -> Reflector {
        Reflector {
            mappings
        }
    }

    pub fn feed_input(&self, index: usize) -> usize {
        for mapping in self.mappings {
            if mapping.0 == index {
                return mapping.1;
            }

            if mapping.1 == index {
                return mapping.0;
            }
        }
        
        panic!("Reflector couldn't find mapping for {}", index);
    }
}

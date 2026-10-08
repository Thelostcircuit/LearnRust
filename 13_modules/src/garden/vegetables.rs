// The `garden::vegetables` module. Its full path is `crate::garden::vegetables`.

#[derive(Debug)]
pub struct Carrot {
    length_cm: u32, // private: only code in this module can touch it directly
}

impl Carrot {
    pub fn new(length_cm: u32) -> Self {
        Carrot { length_cm }
    }

    pub fn length_cm(&self) -> u32 {
        self.length_cm
    }
}

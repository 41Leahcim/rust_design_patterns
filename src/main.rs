#[derive(Default)]
pub struct Second(u64);

impl Second {
    // Constructs a new instance.
    // No self arguments == associated functions
    // The new function is often expected
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

// Same as derive
// Implementing Default (derived or like this), is expected and required for some std functionality
/*impl Default for Second{
    fn default() -> Self {
        Self(0)
    }
}*/

fn main() {}

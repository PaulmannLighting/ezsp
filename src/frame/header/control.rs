#[derive(Debug, Eq, PartialEq)]
pub struct Control {
    low: u8,
    high: u8,
}

impl Control {
    pub const fn new(low: u8, high: u8) -> Self {
        Self { low, high }
    }

    pub const fn low(&self) -> u8 {
        self.low
    }

    pub const fn high(&self) -> u8 {
        self.high
    }
}

impl From<Control> for [u8; 2] {
    fn from(control: Control) -> Self {
        [control.low, control.high]
    }
}

impl From<[u8; 2]> for Control {
    fn from([low, high]: [u8; 2]) -> Self {
        Self::new(low, high)
    }
}

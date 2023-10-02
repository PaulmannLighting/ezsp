#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Counters {
    tx: Tx,
    rx: Rx,
}

impl Counters {
    pub fn tx(&self) -> &Tx {
        &self.tx
    }

    pub fn rx(&self) -> &Rx {
        &self.rx
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Tx {
    data: usize,
    naks: usize,
    acks: usize,
}

impl Tx {
    pub fn data(&self) -> usize {
        self.data
    }

    pub fn naks(&self) -> usize {
        self.naks
    }

    pub fn acks(&self) -> usize {
        self.acks
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Rx {
    data: usize,
    naks: usize,
    acks: usize,
    errors: usize,
}

impl Rx {
    pub fn data(&self) -> usize {
        self.data
    }

    pub fn naks(&self) -> usize {
        self.naks
    }

    pub fn acks(&self) -> usize {
        self.acks
    }

    pub fn errors(&self) -> usize {
        self.errors
    }
}

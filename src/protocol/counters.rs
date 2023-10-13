#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Counters {
    tx: Tx,
    rx: Rx,
}

impl Counters {
    pub const fn new(tx: Tx, rx: Rx) -> Self {
        Self { tx, rx }
    }

    pub const fn tx(&self) -> &Tx {
        &self.tx
    }

    pub const fn rx(&self) -> &Rx {
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
    pub const fn new(data: usize, naks: usize, acks: usize) -> Self {
        Self { data, naks, acks }
    }

    pub const fn data(&self) -> usize {
        self.data
    }

    pub const fn naks(&self) -> usize {
        self.naks
    }

    pub const fn acks(&self) -> usize {
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
    pub const fn new(data: usize, naks: usize, acks: usize, errors: usize) -> Self {
        Self {
            data,
            naks,
            acks,
            errors,
        }
    }

    pub const fn data(&self) -> usize {
        self.data
    }

    pub const fn naks(&self) -> usize {
        self.naks
    }

    pub const fn acks(&self) -> usize {
        self.acks
    }

    pub const fn errors(&self) -> usize {
        self.errors
    }
}

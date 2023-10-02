mod counters;

use counters::Counters;
use serialport::SerialPort;

pub trait Protocol {
    fn start(serial_port: impl SerialPort);

    fn connect(&self);

    fn stop(&self); // setClosing()

    fn close(&self);

    fn enqueue(&self, frame: &[u8]);

    fn send_transaction(&self, transaction: impl Transaction);

    fn counters(&self) -> Counters;
}

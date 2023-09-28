use serialport::SerialPort;

pub trait Protocol {
    fn start(serial_port: SerialPort);

    fn set_closing(&self);

    fn close(&self);

    fn is_alive(&self) -> bool;

    fn queue_frame(&self, request: Request);

    fn connect(&self);
}

//! Echo command implementation.

use ezsp::Utilities;
use ezsp::uart::Uart;
use log::error;
use serialport::SerialPort;

/// Echoes a message.
pub async fn echo<T>(uart: &mut Uart<T>, message: String)
where
    T: SerialPort + 'static,
{
    let mut bytes = heapless::Vec::<u8, { u8::MAX as usize }>::new();

    if bytes
        .extend_from_slice(&message.bytes().collect::<Vec<_>>())
        .is_err()
    {
        error!("Message too long!");
        return;
    };

    match uart.echo(bytes).await {
        Ok(echo) => match String::from_utf8(echo.to_vec()) {
            Ok(echo) => {
                if echo == message {
                    println!("{echo}");
                } else {
                    error!("Echo mismatch: {echo}");
                }
            }
            Err(error) => {
                error!("UTF-8 error: {error}");
            }
        },
        Err(error) => {
            error!("Error echoing message: {error}");
        }
    }
}

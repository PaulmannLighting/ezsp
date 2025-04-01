//! Echo command implementation.

use log::error;

use ezsp::Utilities;
use ezsp::uart::Uart;

/// Echoes a message.
pub async fn echo(uart: &mut Uart, message: String) {
    let mut bytes = heapless::Vec::<u8, { u8::MAX as usize }>::new();

    if bytes.extend_from_slice(&message.bytes().collect::<Vec<_>>()) == Err(()) {
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

use ezsp::Utilities;
use log::{error, info};

pub trait PrintEcho {
    fn print_echo<T>(&mut self, text: T) -> impl Future<Output = ()>
    where
        T: AsRef<[u8]>;
}

impl<U> PrintEcho for U
where
    U: Utilities,
{
    async fn print_echo<T>(&mut self, text: T)
    where
        T: AsRef<[u8]>,
    {
        match self.echo(text.as_ref().iter().copied().collect()).await {
            Ok(echo) => match String::from_utf8(echo.to_vec()) {
                Ok(echo) => {
                    info!("Got echo: {echo}");
                }
                Err(error) => {
                    error!("{error}");
                }
            },
            Err(error) => {
                error!("{error}");
            }
        }
    }
}

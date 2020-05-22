use iced_futures::futures;
use tokio::process::{Child, Command};

// Just a little utility function
pub fn bitcoind<T: ToString>(network: T) -> iced::Subscription<Update> {
    iced::Subscription::from_recipe(ChildProcess {
        network: network.to_string(),
    })
}

pub struct ChildProcess {
    network: String,
}

// Make sure iced can use our download stream
impl<H, I> iced_native::subscription::Recipe<H, I> for ChildProcess
where
    H: std::hash::Hasher,
{
    type Output = Update;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.network.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            State::Start(self.network),
            |state| async move {
                match state {
                    State::Start(network) => {
                        let mut child_result =
                            Command::new("bitcoind").arg("-regtest").spawn();

                        match child_result {
                            Ok(child) => {
                                Some((Update::On, State::On { child }))
                            }
                            Err(_) => Some((Update::Off, State::Off)),
                        }
                    }
                    State::On { mut child } => match child.await {
                        Ok(exit_number) => {
                            println!(
                                "Ok(Some) on child.await: {:?}",
                                exit_number
                            );
                            Some((Update::Off, State::Off))
                        }
                        Err(e) => {
                            println!("Err(e) on child.await: {:?}", e);
                            Some((Update::Off, State::Off))
                        }
                    },
                    State::Off => {
                        // We do not let the stream die, as it would start a
                        // new download repeatedly if the user is not careful
                        // in case of errors.
                        let _: () = iced::futures::future::pending().await;

                        None
                    }
                }
            },
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Update {
    On,
    // Retain StatusCode?
    Off,
}

pub enum State {
    Start(String),
    On { child: Child },
    Off,
}

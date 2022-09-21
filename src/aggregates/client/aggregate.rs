use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use super::{commands::ClientCommand, error::ClientError, events::ClientEvent};

#[derive(Default, Serialize, Deserialize)]
pub struct ClientState {
  messages: Vec<String>,
}

#[async_trait]
impl Aggregate for ClientState {
  type Command = ClientCommand;
  type Event = ClientEvent;
  type Error = ClientError;
  type Services = ();

  fn aggregate_type() -> String {
    "client".to_string()
  }

  async fn handle(
    &self,
    command: Self::Command,
    _: &Self::Services,
  ) -> Result<Vec<Self::Event>, Self::Error> {
    match command {
      Self::Command::Add(msgs) => {
        let messages = vec![self.messages.clone(), msgs]
          .into_iter()
          .flatten()
          .collect::<Vec<String>>();
        Ok(vec![Self::Event::MessagesAdded(messages)])
      }
      Self::Command::Panic => Ok(vec![Self::Event::Paniced]),
    }
  }

  fn apply(&mut self, event: Self::Event) {
    match event {
      Self::Event::MessagesAdded(msgs) => {
        self.messages = msgs;
      }
      Self::Event::Paniced => {}
    }
  }
}

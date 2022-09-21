use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClientEvent {
  MessagesAdded(Vec<String>),
  Paniced,
}

impl DomainEvent for ClientEvent {
  fn event_type(&self) -> String {
    match self {
      ClientEvent::MessagesAdded(_) => "Messages added".to_owned(),
      ClientEvent::Paniced => "Paniced".to_owned(),
    }
  }

  fn event_version(&self) -> String {
    "1.0".to_string()
  }
}

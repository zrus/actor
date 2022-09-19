mod actor;
mod client;
mod commands;
mod error;
mod state;
mod traits;

use anyhow::Result;
use bastion::prelude::*;

use crate::{
  client::{AskClient, ClientActor, ClientState, TellClient},
  state::State,
  traits::TActor,
};

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  Bastion::init();
  Bastion::start();

  let state = State::<ClientState>::default();
  ClientActor::default().run(&state.downgrade())?;
  let client = Distributor::named("client");

  tokio::time::sleep(std::time::Duration::from_secs(1)).await;
  client.tell_one(TellClient::Add("Hello, I am Tuong"))?;

  let answer: Vec<String> = client.request(AskClient::Messages).await??;
  println!("Messages: {answer:?}");

  Bastion::block_until_stopped();
  Ok(())
}

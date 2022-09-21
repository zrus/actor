mod actors;
mod aggregates;

use anyhow::Result;
use bastion::prelude::*;

use crate::{
  actors::{client::ClientActor, state::State, traits::TActor},
  aggregates::client::aggregate::ClientState,
};

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  Bastion::init();
  Bastion::start();

  let state = State::<ClientState>::default();
  ClientActor::default()
    .with_restart_strategy(RestartStrategy::new(
      RestartPolicy::Always,
      ActorRestartStrategy::Immediate,
    ))
    .run(&state.downgrade())?;

  tokio::time::sleep(std::time::Duration::from_secs(1)).await;
  let client = Distributor::named("client");

  Bastion::block_until_stopped();
  Ok(())
}

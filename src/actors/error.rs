use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActorError {
  #[error("Unable to initialize supervisor")]
  UnableToInitSupervisor,
  #[error("Unable to initialize children")]
  UnableToInitChildren,
  #[error("Supervisor stop error")]
  StopError,
  #[error("Supervisor kill error")]
  KillError,
  #[error("Add executor failed")]
  AddExecutorFailed,
}

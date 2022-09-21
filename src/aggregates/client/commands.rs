#[derive(Debug)]
pub enum ClientCommand {
  Add(Vec<String>),
  Panic,
}

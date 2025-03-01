pub trait Command {
    fn to_command_string(&self) -> String;
    fn receive_reply(&self) -> bool;
}
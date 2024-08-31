pub trait InstructionInfo {
    fn get_target(&self) -> String;
    fn get_value(&self) -> String;
}
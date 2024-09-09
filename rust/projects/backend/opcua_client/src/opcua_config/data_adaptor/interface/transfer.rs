pub trait InstructionInfo : Send{
    fn get_target(&self) -> String;
    fn get_value(&self) -> String;
}
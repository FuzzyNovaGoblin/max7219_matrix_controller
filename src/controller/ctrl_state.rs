use crate::symbols::symbol::Symbol;

use super::ctrl_state_atributes::Orientation;


pub struct CtrlState {
    char_sequence: String,
    symbol_sequence: Vec<Symbol>,
    module_count: u8,
    animation_step: usize,
    animation_length: usize,
    orientation: Orientation,
}

impl CtrlState {
    pub fn new(module_count: u8) -> CtrlState {
        CtrlState {
            char_sequence: String::new(),
            symbol_sequence: Vec::new(),
            module_count,
            animation_step: 0,
            animation_length: 0,
            orientation: Orientation::Horizontal,

        }
    }

    pub fn set_char_sequence(&mut self, new_sequence: String){
        self.char_sequence = new_sequence;

    }

    fn build_symbol_sequence(){

    }


}

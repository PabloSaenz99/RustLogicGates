use crate::logic_gate_base::{LogicGate, LogicGateOperations};

pub struct OR { pub data: LogicGate}

impl LogicGateOperations<OR> for OR {
    fn calculate_output(&mut self) -> bool {
        self.data.set_output(self.data.get_left_input() || self.data.get_right_input());
        self.data.get_output()
    }

    fn new() -> OR {
        OR{data: LogicGate::new("OR".to_string())}
    }
}
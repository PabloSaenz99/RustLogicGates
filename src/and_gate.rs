use crate::logic_gate_base::{LogicGate, LogicGateOperations};

pub struct AND {
    pub data: LogicGate
}

impl LogicGateOperations<AND> for AND {
    fn calculate_output(&mut self) -> bool {
        self.data.set_output(self.data.get_left_input() && self.data.get_right_input());
        self.data.get_output()
    }

    fn new() -> AND {
        AND{data: LogicGate::new("AND".to_string())}
    }
}
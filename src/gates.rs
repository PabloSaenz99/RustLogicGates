use crate::logic_gate_base::LogicGate;

pub  enum GateTypes {AND, OR, IN}

impl GateTypes {
    pub fn to_string(&self) -> String {
        match self {
            GateTypes::AND => "AND".to_string(),
            GateTypes::OR => "OR".to_string(),
            GateTypes::IN => "IN".to_string(),
        }
    }
}

pub fn calculate_output(gate: &LogicGate) -> bool {
    return match gate.get_type() {
        GateTypes::AND => gate.get_left_input() && gate.get_right_input(),
        GateTypes::OR => gate.get_left_input() || gate.get_right_input(),
        GateTypes::IN => gate.get_left_input()
    };
}

pub fn print_gate(gate: &LogicGate) {
    println!("{}", gate.get_string()[0]);
    println!("{}", gate.get_string()[1]);
    println!("{}", gate.get_string()[2]);
}


/* pub fn calculate_output(gate: &mut LogicGate) -> bool {
    match gate.get_type() {
        GATES::AND => gate.set_output(gate.get_left_input() && gate.get_right_input()),
        GATES::OR => gate.set_output(gate.get_left_input() || gate.get_right_input()),
        GATES::IN => gate.set_output(gate.get_left_input())
    }
    return gate.get_output();
} */
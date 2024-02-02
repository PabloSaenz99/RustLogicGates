#[derive(Debug)]
pub  enum GateTypes {AND, OR, NOT}

impl GateTypes {
	pub fn to_string(&self) -> String {
		match self {
			GateTypes::AND => "AND".to_string(),
			GateTypes::OR => "OR".to_string(),
			GateTypes::NOT => "NOT".to_string(),
		}
	}
}

/* pub fn print_gate(gate: &LogicGate) {
	println!("{}", gate.get_string()[0]);
	println!("{}", gate.get_string()[1]);
	println!("{}", gate.get_string()[2]);
} */


/* pub fn calculate_output(gate: &mut LogicGate) -> bool {
	match gate.get_type() {
		GATES::AND => gate.set_output(gate.get_left_input() && gate.get_right_input()),
		GATES::OR => gate.set_output(gate.get_left_input() || gate.get_right_input()),
		GATES::IN => gate.set_output(gate.get_left_input())
	}
	return gate.get_output();
} */
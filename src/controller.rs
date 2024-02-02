use crate::{gates::GateTypes, logic_gate_base::LogicGate};
use std::cell::RefCell;

pub struct Controller {
	outputs: Vec<LogicGate>,
	inputs: Vec<LogicGate>,
	all_gates: Vec<LogicGate>
}

impl Controller {
	pub fn new() -> RefCell<Controller>{
		RefCell::new(Controller{ outputs: vec![], inputs: vec![], all_gates: vec![] })
	}

	pub fn add_new_gate(&mut self, _type: GateTypes) {
		self.add_gate(LogicGate::new(_type));
	}

	pub fn add_gate(&mut self, gate: LogicGate) {
		self.all_gates.push(gate);
		/* let l = gate.get_left_input_connection();
		let r = gate.get_right_input_connection();
		if let (Some(_), Some(_)) = (l, r) {
			self.inputs.push(gate);
		} */
		// if(gate.get_output_connection().len() )
	}

	/* pub fn get_gate(self, i: usize) -> Option<LogicGate> {
		self.all_gates.get(i)
	} */

	/* pub fn join_to_left_gate(&mut self, mut left: &Box<LogicGate>, mut to: Box<LogicGate>) {
		to.set_left_input_connection(Some(left));
		left.add_output_connection(*to);
	}
	pub fn join_to_right_gate(&mut self, mut right: Box<LogicGate>, mut to: Box<LogicGate>) {
		to.set_left_input_connection(Some(right));
		right.add_output_connection(*to);
	} */

	pub fn execute(&self) {
		for i in self.inputs.iter() {
			/* i.calculate_output();
			while let Some(o) = i.get_output_connection() {
				o.
			} */
		}
	}

	pub fn print(&self) {
		let mut inputs = Vec::<[String; 3]>::with_capacity(self.all_gates.len());
		for g in &self.all_gates {
			inputs.push(g.get_string());
		}
		for j in 0..3 {
			for i in 0..inputs.len() {
				if j == 0 {
					print!("[{:^11}]", inputs[i][j]);
				} else if j == 1 {
					print!("|{:^11}|", format!("{} - {:?}", i, inputs[i][j]));
				} else{
					print!("[{:^11}]", inputs[i][j]);
				}
			}
			println!();
		}
	}
}
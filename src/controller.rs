use crate::{gates::GateTypes, logic_gate_base::LogicGate};
use std::{cell::RefCell, rc::Rc};

pub struct Controller {
	outputs: Vec<Rc<RefCell<LogicGate>>>,
	inputs: Vec<Rc<RefCell<LogicGate>>>,
	all_gates: Vec<Rc<RefCell<LogicGate>>>
}

impl Controller {
	pub fn new() -> RefCell<Controller>{
		RefCell::new(Controller{ outputs: vec![], inputs: vec![], all_gates: vec![] })
	}

	pub fn add_new_gate(&mut self, _type: GateTypes) {
		// self.add_gate(LogicGate::new(_type));
	}

	pub fn add_gate(&mut self, gate: LogicGate) {
		let ref_gate = Rc::new(RefCell::new(gate));
		self.all_gates.push(ref_gate.clone());
		
		let binding = ref_gate.clone();
  		let binding = binding.borrow();
  		let l = binding.get_left_input_connection();
		let binding = ref_gate.clone();
  		let binding = binding.borrow();
  		let r = binding.get_right_input_connection();
		if let (Some(_), Some(_)) = (l, r) {
			self.inputs.push(ref_gate.clone());
		}
		/* for o in gate.get_output_connection() {
			
		} */
		// if(gate.get_output_connection().len() )
	}

	/* pub fn get_gate(self, i: usize) -> Option<LogicGate> {
		self.all_gates.get(i)
	} */

	pub fn join_to_left_gate(&mut self, left: Rc<RefCell<LogicGate>>, to: Rc<RefCell<LogicGate>>) {
		to/* .clone() */.borrow_mut().set_left_input_connection(Some(left.clone()));
		left.borrow_mut().add_output_connection(Some(to.clone()));
	}
	pub fn join_to_right_gate(&mut self, right: Rc<RefCell<LogicGate>>, to: Rc<RefCell<LogicGate>>) {
		to.borrow_mut().set_left_input_connection(Some(right.clone()));
		right.borrow_mut().add_output_connection(Some(to.clone()));
	}

	pub fn execute(&self) {
		for i in self.outputs.iter() {
			/* i.calculate_output();
			while let Some(o) = i.get_output_connection() {
				o.
			} */
		}
	}

	pub fn print(&self) {
		let mut inputs = Vec::<[String; 3]>::with_capacity(self.all_gates.len());
		for g in &self.all_gates {
			inputs.push(g.borrow().get_string());
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
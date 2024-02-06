use crate::{gates::GateTypes, logic_gate_base::LogicGate};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Controller {
	outputs: Vec<Rc<RefCell<LogicGate>>>,
	inputs: Vec<Rc<RefCell<LogicGate>>>,
	all_gates: HashMap<u128, Rc<RefCell<LogicGate>>>
}

impl Controller {
	pub fn new() -> RefCell<Controller>{
		RefCell::new(Controller{ outputs: vec![], inputs: vec![], all_gates: HashMap::new() })
	}

	pub fn add_new_gate(&mut self, _type: GateTypes) {
		self.add_gate(LogicGate::new(_type));
	}

	pub fn add_gate(&mut self, gate: LogicGate) {
		let ref_gate = Rc::new(RefCell::new(gate));
		self.all_gates.insert(ref_gate.clone().borrow().get_id(), ref_gate.clone());

		let binding = ref_gate.clone();
  		let binding = binding.borrow();
  		let l = binding.get_left_input_connection();
		let binding = ref_gate.clone();
  		let binding = binding.borrow();
  		let r = binding.get_right_input_connection();
		match (l, r) {
    		(Some(_), Some(_)) => {},
			_ => self.inputs.push(ref_gate.clone()),
		}
		if ref_gate.clone().borrow().get_output_connection().len() == 0 {
			self.outputs.push(ref_gate.clone());
		}
		else {
			for o in ref_gate.clone().borrow().get_output_connection() {
				if let None = o {	//If is there is a None, add this to outputs
					self.outputs.push(ref_gate.clone());
				}
			}
		}
	}

	pub fn get_gate(self, i: u128) -> Option<Rc<RefCell<LogicGate>>> {
		self.all_gates.get(&i).cloned()
	}

	pub fn join_to_left_gate(&mut self, left: Rc<RefCell<LogicGate>>, to: Rc<RefCell<LogicGate>>) {
		to/* .clone() */.borrow_mut().set_left_input_connection(Some(left.clone()));
		left.borrow_mut().add_output_connection(Some(to.clone()));
	}
	pub fn join_to_right_gate(&mut self, right: Rc<RefCell<LogicGate>>, to: Rc<RefCell<LogicGate>>) {
		to.borrow_mut().set_left_input_connection(Some(right.clone()));
		right.borrow_mut().add_output_connection(Some(to.clone()));
	}

	pub fn execute(&self) {
		for i in &self.outputs {
			i.clone().borrow().calculate_output();
		}
	}

	pub fn print_tree(&self) {
		let mut vertical = Vec::new();
		for o in &self.outputs {
			self.print_recursive(o.clone(), &mut vertical, 0)
		}

		for v in vertical {
			for i in 0..5 {
				for s in &v {
					print!("|{:^19}|", s.1[i]);
				};
				println!();
			}
		}
	}

	fn print_recursive(&self, gate: Rc<RefCell<LogicGate>>, vertical: &mut Vec<HashMap<u128, [String; 5]>>, height: usize) {
		if height >= vertical.len() {
			vertical.push(HashMap::new());
		}

		let binding = gate.clone();
  		let current_gate= binding.borrow();
		//Used to reduce backtracking
		if !vertical[height].contains_key(&gate.borrow().get_id()) {
			match (current_gate.get_left_input_connection(), current_gate.get_right_input_connection()) {
				(None, None) => {}
				(None, Some(r)) => self.print_recursive(r.clone(), vertical, height + 1),
				(Some(l), None) => self.print_recursive(l.clone(), vertical, height + 1),
				(Some(l), Some(r)) => {
					self.print_recursive(l.clone(), vertical, height + 1);
					self.print_recursive(r.clone(), vertical, height + 1);
				}
			}
			current_gate.calculate_output();
			// vertical[height].insert(gate, current_gate.get_string());
			vertical[height].insert(gate.borrow().get_id(), current_gate.get_full_string());
		}
	}

	pub fn print_all(&self) {
		let mut inputs = Vec::<[String; 3]>::with_capacity(self.all_gates.len());
		for g in &self.all_gates {
			inputs.push( g.1.borrow().get_string());
		}

		println!("Nº gates: {}", self.all_gates.len());
		println!("Nº inputs: {}", self.inputs.len());
		println!("Nº outputs: {}", self.outputs.len());
		println!();

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
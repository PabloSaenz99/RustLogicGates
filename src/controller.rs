use crate::{gates::GateTypes, input_utils::{error_option, read_terminal, InputOptions}, logic_gate_base::{LogicGate, RcLogicGate}};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Controller {
	outputs: HashMap<u128, RcLogicGate>,
	inputs: HashMap<u128, RcLogicGate>,
	all_gates: HashMap<u128, RcLogicGate>
}

impl Controller {
	pub fn new() -> RefCell<Controller>{
		RefCell::new(Controller{ outputs: HashMap::new(), inputs: HashMap::new(), all_gates: HashMap::new() })
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
			_ => {self.inputs.insert(ref_gate.borrow().get_id(), ref_gate.clone());},
		};
		if ref_gate.borrow().get_output_connection().len() == 0 {
			self.outputs.insert(ref_gate.borrow().get_id(), ref_gate.clone());
		}
		else {
			for o in ref_gate.clone().borrow().get_output_connection().values() {
				if let None = o {	//If is there is a None, add this to outputs
					self.outputs.insert(ref_gate.borrow().get_id(), ref_gate.clone());
				}
			}
		}
	}

	pub fn get_gate(&self, i: u128) -> Option<RcLogicGate> {
		self.all_gates.get(&i).cloned()
	}

	pub fn join_to_left_gate(&mut self, left: RcLogicGate, to: RcLogicGate) {
		to.borrow_mut().set_left_input_connection(Some(left.clone()));
		left.borrow_mut().add_output_connection(to.clone().borrow().get_id(), Some(to.clone()));
	}
	pub fn join_to_right_gate(&mut self, right: RcLogicGate, to: RcLogicGate) {
		to.borrow_mut().set_left_input_connection(Some(right.clone()));
		right.borrow_mut().add_output_connection(to.clone().borrow().get_id(),Some(to.clone()));
	}

	pub fn remove_from_gate_to_left(&mut self, from: RcLogicGate, to: Option<RcLogicGate>) {
		from.borrow_mut().remove_output_connection(&to);
		if to.is_some() {
			let t = to.clone().unwrap();
			t.borrow_mut().set_left_input_connection(None);
			//If there are no connections, add to inputs
			if t.borrow().get_right_input_connection().is_none() {
				self.inputs.insert(t.borrow().get_id(), t.clone());
			}
		}
	}
	pub fn remove_from_gate_to_right(&mut self, from: RcLogicGate, to: Option<RcLogicGate>) {
		from.borrow_mut().remove_output_connection(&to);
		if to.is_some() {
			let t = to.clone().unwrap();
			t.borrow_mut().set_right_input_connection(None);
			//If there are no connections, add to inputs
			if t.borrow().get_left_input_connection().is_none() {
				self.inputs.insert(t.borrow().get_id(), t.clone());
			}
		}
	}

	pub fn execute(&self) {
		for i in self.outputs.values() {
			i.clone().borrow().calculate_output();
		}
	}

	pub fn print_gate(&self, gate: Option<RcLogicGate>) {
		match gate {
			Some(g) => {
				for s in g.borrow().get_string() {
					println!("|{:^19}|", s);
				}
			}
			None => println!("Gate not found!"),
		}
	}

	pub fn print_tree(&self) {
		let mut vertical = Vec::new();
		for o in self.outputs.values() {
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

	fn print_recursive(&self, gate: RcLogicGate, vertical: &mut Vec<HashMap<u128, [String; 5]>>, height: usize) {
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
			vertical[height].insert(gate.borrow().get_id(), current_gate.get_string());
		}
	}

	pub fn parse_options(&mut self) {
		let input = read_terminal().trim().to_string();
		match InputOptions::from(input.clone()) {
			InputOptions::Exit => std::process::exit(0),
			InputOptions::New(_type) => self.add_new_gate(_type),
			InputOptions::Show(opt) =>
				match opt.as_str() {
					"all" => self.print_tree(),
					_=> self.print_gate(self.get_gate(opt.parse().unwrap_or(0))),
				}
			InputOptions::Link(opt, input_gate, output_gate) =>
				match (self.get_gate(input_gate), self.get_gate(output_gate)) {
					(None, None) => error_option("Input and output not valid!", self),
					(None, Some(_)) => error_option(input_gate.to_string().as_str(), self),
					(Some(_), None) => error_option(output_gate.to_string().as_str(), self),
					(Some(i), Some(o)) =>
						match opt.as_str() {
							"left" => self.join_to_left_gate(i, o),
							"right" => self.join_to_right_gate(i, o),
							_=> error_option(&opt, self)
						},
				},
			InputOptions::Unlink(position, from, to) =>
				match (self.get_gate(from), self.get_gate(to)) {
					(None, None) => error_option("There must be an input", self),
					(None, Some(_)) => error_option("There must be an input", self),
					(Some(out), None) => {
						self.outputs.remove(&out.borrow().get_id());
						println!("Removed output for gate {}!", out.borrow().get_id());
					}
					(Some(out), Some(input)) =>
					match position.as_str() {
						"left" => self.remove_from_gate_to_left(out, Some(input)),
						"right" => self.remove_from_gate_to_right(out, Some(input)),
						_=> error_option(&position, self)
					}
				}
			InputOptions::Input(id, mode, value) =>
				match self.get_gate(id) {
					Some(gate) =>
						match mode.as_str() {
							"left" => gate.borrow_mut().set_left_input(value),
							"right" => gate.borrow_mut().set_right_input(value),
							_ => error_option("must be left or right", self)
						}
					None => error_option("id", self),
				},
			InputOptions::Error => error_option(&input, self),
		};
		println!();
	}
}
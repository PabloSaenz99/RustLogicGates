use crate::gates::GateTypes;
use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct LogicGate {
	_type: GateTypes,
	id: u128,

	left_input: bool,
	right_input: bool,
	// output: bool,

	left_input_connection: Option<Rc<RefCell<LogicGate>>>,
	right_input_connection: Option<Rc<RefCell<LogicGate>>>,
	output_connection: Vec<Option<Rc<RefCell<LogicGate>>>>,
}

impl LogicGate {
	//Default constructor
	pub fn new(_type: GateTypes) -> LogicGate {
		LogicGate::full_gate(_type, false, false, None, None)
	}

	pub fn from(_type: GateTypes, left_input: bool, right_input: bool) -> LogicGate {
		LogicGate::full_gate(_type, left_input, right_input, None, None)
	}

	pub fn with(_type: GateTypes,
			left_input_connection: Option<Rc<RefCell<LogicGate>>>,
			right_input_connection: Option<Rc<RefCell<LogicGate>>>) -> LogicGate {
		LogicGate::full_gate(_type, false, false, left_input_connection, right_input_connection)
	}

	fn full_gate(_type: GateTypes, left_input: bool, right_input: bool,
		left_input_connection: Option<Rc<RefCell<LogicGate>>>,
		right_input_connection: Option<Rc<RefCell<LogicGate>>>) -> LogicGate {
		let mut l = LogicGate {
			_type, id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
			left_input, right_input,
			left_input_connection, right_input_connection, output_connection: Vec::new()
		};
		l.set_left_input(l.get_left_input());
		l.set_right_input(l.get_right_input());
		l.calculate_output();
		return l;
	}

	pub fn get_type(&self) -> &GateTypes { &self._type }
	pub fn get_id(&self) -> u128 { self.id }
	pub fn get_left_input(&self) -> bool { self.left_input }
	pub fn get_right_input(&self) -> bool { self.right_input }
	pub fn get_output(&self) -> bool { self.calculate_output() }

	pub fn get_string(&self) -> [String; 5] {
		[self.get_output().to_string(),
		self.get_type().to_string(),
		self.get_id().to_string(),
		self.get_left_input().to_string() + "-" + &self.get_right_input().to_string(),
		format!("{:.8}-{:.8}", self.get_id_string(self.get_left_input_connection().clone()),
			&self.get_id_string(self.get_right_input_connection().clone()))]
	}

	fn get_id_string(&self, opt: Option<Rc<RefCell<LogicGate>>>) -> String{
		match opt {
			Some(o) => o.borrow().get_id().to_string(),
			None => "None".to_string()
		}
	}

	pub fn set_left_input(&mut self, new_input: bool) {
		match &self.left_input_connection {
			Some(a) => {
				println!("Error, input setted, using its value");
				self.left_input = a.borrow().get_output();
			}
			None => self.left_input = new_input
		}
	}
	pub fn set_right_input(&mut self, new_input: bool) {
		match &self.right_input_connection {
			Some(a) => {
				println!("Error, input setted, using its value");
				self.right_input = a.borrow().get_output();
			}
			None => self.right_input = new_input
		}
	}

	pub fn get_left_input_connection(&self) -> &Option<Rc<RefCell<LogicGate>>> { &self.left_input_connection }
	pub fn get_right_input_connection(&self) -> &Option<Rc<RefCell<LogicGate>>> { &self.right_input_connection }
	pub fn get_output_connection(&self) -> &Vec<Option<Rc<RefCell<LogicGate>>>> { &self.output_connection }

	pub fn  set_left_input_connection<'a>(&mut self, new_input: Option<Rc<RefCell<LogicGate>>>) {
		self.left_input_connection = new_input;
		self.set_left_input(self.left_input);
	}
	pub fn set_right_input_connection(&mut self, new_input: Option<Rc<RefCell<LogicGate>>>) {
		self.right_input_connection = new_input;
		self.set_right_input(self.right_input);
	}
	pub fn set_output_connection(&mut self, new_output: Vec<Option<Rc<RefCell<LogicGate>>>>) { self.output_connection = new_output; }
	pub fn add_output_connection(&mut self, new_output: Option<Rc<RefCell<LogicGate>>>) { self.output_connection.push(new_output); }

	pub fn calculate_output(&self) -> bool {
		return match self.get_type() {
			GateTypes::AND => self.get_left_input() && self.get_right_input(),
			GateTypes::OR => self.get_left_input() || self.get_right_input(),
			GateTypes::NOT => self.get_left_input()
		};
	}

	pub fn print_gate(&self) {
		let gate_str = self.get_string();
		println!("{}", gate_str[0]);
		println!("{}", gate_str[1]);
		println!("{}", gate_str[2]);
	}
}
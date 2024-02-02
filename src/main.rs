use std::rc::Rc;

use controller::Controller;

use crate::{gates::GateTypes, logic_gate_base::LogicGate};

mod logic_gate_base;
pub mod gates;
pub mod controller;

fn main() {
	/* let a = LogicGate::from(GateTypes::AND, true, true);
	let o = LogicGate::with(GateTypes::OR, Some(Box::new(a)), None);
	let a = o.get_left_input_connection();

	o.print_gate();
	if let Some(i) = a {
		i.print_gate();
	} */

	let ctrl = Controller::new();
	ctrl.borrow_mut().add_new_gate(GateTypes::AND);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);
	ctrl.borrow().print();
}
use controller::Controller;

use crate::gates::GateTypes;

mod logic_gate_base;
pub mod gates;
pub mod controller;

fn main() {
	let ctrl = Controller::new();
	ctrl.borrow_mut().add_new_gate(GateTypes::AND);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);
	ctrl.borrow().print_all();

	println!("-----------");
	ctrl.borrow().print_tree();
}
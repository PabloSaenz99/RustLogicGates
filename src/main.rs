use controller::Controller;

use crate::{gates::GateTypes, input_utils::{read_options, show_options}};

mod logic_gate_base;
pub mod gates;
pub mod controller;
mod input_utils;

fn main() {
	let ctrl = Controller::new();
	ctrl.borrow_mut().add_new_gate(GateTypes::AND);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);
	ctrl.borrow_mut().add_new_gate(GateTypes::OR);
	ctrl.borrow().print_all();

	println!("-----------");
	ctrl.borrow().print_tree();

	show_options();
	read_options();
}
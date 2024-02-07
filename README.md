# RustLogicGates
#### Rust project used to create a logic gate simulator.

In this project I will try to create a logic gate simulator using my knowledge using Rust and update and refactor it as long as I learn new skills.

## Features
- Create predefined gates such as AND, OR, NOT
- Show an specific gateor all getes in a pseudo-tree view
- Link one gate with another and update de output depending on the inputs of the previous gates
- Unlink two gates
- Set input values for gates whose inputs are mnot linked with another

### Commands:
- Exit
- Show [gate_id || all] -> Shows an specific gate or all the gates as a tree
- New [gate_type] -> [gate_type] can be AND, OR, NOT.
- Input [gate_id] [left || right], [bool] -> Sets the left or right input of the [gate_id] with the specified [bool] value or false if its an incorrect value.
- Link [left || right] [from] [to] -> Links the right or left input of the [to] gate with the output of the [from] gate
- Unlink [left || right || none] [from] [to | none] -> Unlinks the left or right input of the [to] gate from the output of the [from] gate. If the [to] param is [none] or incorrect, the [left || right || none] param will be ignored and it will unlink the [from] gate from the tree final outputs.

## Roadmap
- Add an option to save current simulator to a file.
- Read a simulator from file.
- Create custom gates and save them and use them.
- Refactor the tree view
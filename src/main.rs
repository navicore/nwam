use std::collections::HashMap;

// Define a tag to represent the type of heap cell.
#[derive(Debug, Clone, PartialEq)]
enum Tag {
    Ref,
    Str,
}

// Define a heap cell to store either a reference or a structure.
#[derive(Debug, Clone)]
enum HeapCell {
    // REF tag with an address pointing to a heap index (or self-referential for unbound variables).
    Ref { tag: Tag, address: usize },
    // STR tag with an address pointing to the functor.
    Str { tag: Tag, functor_address: usize },
    // Functor representation (functor and arity).
    Functor { name: String, arity: usize },
}

// Define the heap as a vector of heap cells.
struct Heap {
    cells: Vec<HeapCell>,
}

impl Heap {
    // Create a new, empty heap.
    fn new() -> Self {
        Heap { cells: Vec::new() }
    }

    // Add a new unbound variable to the heap.
    fn add_variable(&mut self) -> usize {
        let address = self.cells.len();
        self.cells.push(HeapCell::Ref {
            tag: Tag::Ref,
            address, // Self-referential for unbound variable.
        });
        address
    }

    // Add a new structure to the heap.
    fn define_functor_structure(&mut self, functor: String, arity: usize) -> usize {
        // Create a functor cell.
        let functor_address = self.cells.len();
        self.cells.push(HeapCell::Functor {
            name: functor,
            arity,
        });

        // Create a structure (STR) cell pointing to the functor.
        let str_address = self.cells.len();
        self.cells.push(HeapCell::Str {
            tag: Tag::Str,
            functor_address,
        });

        // Add arity number of unbound variables for subterms.
        for _ in 0..arity {
            self.add_variable();
        }

        str_address
    }

    // Debug function to print the entire heap.
    fn print_heap(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            println!("Address {}: {:?}", i, cell);
        }
    }

    // Put a structure on the heap and store it in a register.
    fn init_structure_in_register(
        &mut self,
        registers: &mut Registers,
        functor: String,
        arity: usize,
        reg_index: usize,
    ) {
        let address = self.define_functor_structure(functor, arity);
        registers.set(
            reg_index,
            HeapCell::Str {
                tag: Tag::Str,
                functor_address: address,
            },
        );
    }

    // Set a variable on the heap and store its reference in a register.
    fn init_variable_in_register(&mut self, registers: &mut Registers, reg_index: usize) {
        let address = self.add_variable();
        registers.set(
            reg_index,
            HeapCell::Ref {
                tag: Tag::Ref,
                address,
            },
        );
    }

    // Set a value on the heap from a register.
    fn assign_value_from_register(&mut self, registers: &Registers, reg_index: usize) {
        if let Some(value) = registers.get(reg_index) {
            self.cells.push(value.clone());
        }
    }
}

// Define registers to temporarily store heap cells.
struct Registers {
    registers: HashMap<usize, HeapCell>,
}

impl Registers {
    // Create new, empty registers.
    fn new() -> Self {
        Registers {
            registers: HashMap::new(),
        }
    }

    // Set a value in a register.
    fn set(&mut self, index: usize, value: HeapCell) {
        self.registers.insert(index, value);
    }

    // Get a value from a register.
    fn get(&self, index: usize) -> Option<&HeapCell> {
        self.registers.get(&index)
    }
}

fn main() {
    // Create a new heap.
    let mut heap = Heap::new();
    let mut registers = Registers::new();

    // Put a structure with a functor "f" and arity 2 in register X1.
    heap.init_structure_in_register(&mut registers, "f".to_string(), 2, 1);

    // Set a variable in register X2.
    heap.init_variable_in_register(&mut registers, 2);

    // Set a value from register X2 onto the heap.
    heap.assign_value_from_register(&registers, 2);

    // Print the heap to see the current state.
    heap.print_heap();
}

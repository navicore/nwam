use core::fmt;

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
    fn add_structure(&mut self, functor: String, arity: usize) -> usize {
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
}

fn main() {
    // Create a new heap.
    let mut heap = Heap::new();
    println!("print heap:");
    heap.print_heap();

    println!();
    // Add a variable to the heap.
    let var_address = heap.add_variable();
    println!("Added variable at address: {}", var_address);
    println!("print var:");
    heap.print_heap();

    println!();
    // Add a structure with a functor "f" and arity 2.
    let struct_address = heap.add_structure("f".to_string(), 2);
    println!("Added structure at address: {}", struct_address);
    println!("print struct:");
    heap.print_heap();
}

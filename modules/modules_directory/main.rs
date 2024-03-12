// modules_directory/main.rs

mod foo; // Import the module foo

use foo::Bar; // Import the struct Bar from the module foo

fn main() {
    foo::do_foo();
    Bar::hello();
}

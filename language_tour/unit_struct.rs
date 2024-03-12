/* unit_struct.rs

A unit struct is a struct without any fields.
Unit structs can useful for generics, or when you don't need to store any data.
(representing a type as a value, for example)
They behave similarly to (), the unit type.
*/

struct Dummy; // unit struct

fn main() {
    let value = Dummy;
}

// slices.rs

// because we know the size of this array at compile time [u8; 4]
// the values will initially be stored on the stack

fn main() {
    // create an array of 4 u8 numbers on the stack
    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        // create a slice of all the numbers by borrowing the array
        let all: &[u8] = &numbers[..]; // will be stored on the heap!
        println!("All of them: {:?}", all);
    }
    {
        // create a slice of the first two numbers by borrowing the array
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100; // modify the first number of the slice
        first_two[1] = 99; // modify the second number of the slice
    }

    println!("Look, I can modify these through slices: {:?}", numbers);
}

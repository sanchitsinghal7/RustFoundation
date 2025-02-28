pub fn tuple_demo() {
    // Defining a tuple with mixed data types
    let coordinates: (i32, i32) = (82, 64); // Tuple of two integers
    let score: (&str, i32) = ("Team A", 12); // Tuple containing a string and an integer

    // Accessing tuple elements using dot notation
    println!("Tuple Demo:");
    println!("Coordinates: ({}, {})", coordinates.0, coordinates.1);
    println!("Score: Team {} - Points {}", score.0, score.1);

    // Destructuring a tuple
    let (x, y) = coordinates;
    let (team, points) = score;

    println!("Destructured Values:");
    println!("x = {}, y = {}", x, y);
    println!("Team: {}, Points: {}", team, points);
}

pub fn array_demo() {
    // Defining an array with known length
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Fixed-size array

    // Initializing an array with the same value
    let repeated_values: [i32; 3] = [0; 3]; // Equivalent to [0, 0, 0]

    println!("\nArray Demo:");
    println!("Numbers Array: {:?}", numbers);
    println!("Repeated Values Array: {:?}", repeated_values);

    // Accessing array elements
    println!("First Element: {}", numbers[0]);
    println!("Last Element: {}", numbers[numbers.len() - 1]);

    // Iterating through the array
    println!("Iterating through the Numbers array:");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();
}

pub fn slice_demo() {
    let array: [i32; 5] = [10, 20, 30, 40, 50]; // Fixed-size array

    // Borrowing a slice from the array
    let slice: &[i32] = &array[1..4]; // Includes index 1 to 3 (not 4)

    println!("\nSlice Demo:");
    println!("Original Array: {:?}", array);
    println!("Slice: {:?}", slice);

    // Modifying the array (slices reflect this)
    let mut mutable_array: [i32; 5] = [5, 10, 15, 20, 25];
    let mutable_slice: &mut [i32] = &mut mutable_array[2..4];

    mutable_slice[0] *= 2; // Doubling the first element of the slice
    mutable_slice[1] += 5; // Incrementing the second element

    println!("Modified Array after Slice Mutation: {:?}", mutable_array);
}

use std::collections::HashMap;

pub fn hashmap_demo() {
    let mut scores = HashMap::new();

    // Inserting values into the HashMap
    scores.insert("Team A", 30);
    scores.insert("Team B", 25);
    scores.insert("Team C", 40);

    println!("HashMap Demo:");
    println!("Scores: {:?}", scores);

    // Accessing values
    let team = "Team B";
    match scores.get(team) {
        Some(score) => println!("{}'s Score: {}", team, score),
        None => println!("{} not found!", team),
    }

    // Checking if a key exists
    if scores.contains_key("Team A") {
        println!("Team A is in the HashMap.");
    }

    // Removing an entry
    scores.remove("Team B");
    println!("After removing Team B: {:?}", scores);
}



fn main() {
    tuple_demo();
    array_demo();
    slice_demo();
}
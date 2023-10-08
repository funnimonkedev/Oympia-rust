# Oympia-rust
Olympia-rust is a very simple rust terminal game that I made, I have made a guide explaining how the code works to go along with it, this is a great way to learn basic scope, if statements, loop, handling user input, and mutability and immutability.

### --tutorial--


This code is a simple text-based game written in Rust where the user simulates a gym workout. The user can perform various actions such as train, bench, shop, and flex, each of which affects the user's energy, max energy, strength, and money values.

First, let's break down the program step by step:

1. Variable Initialization

``` rust
let mut energy: i128 = 10;
let mut max_energy: i128 = 10;
let mut strength: i128 = 1;
let mut money: i128 = 0;
```
Here, four mutable variables are declared and initialized: `energy`, `max_energy`, `strength`, and `money`. Mutable variables in Rust are variables whose value can be changed rust-lang.org. The `i128` type is a signed 128-bit integer.

2. Game Introduction

``` rust
println!("Welcome to the gym. type 'train' to start training, 'bench' to go to bench, 'shop' to go to shop, and 'flex' to flex on weaker gym people");
```
This line outputs a welcome message to the user, explaining the possible actions they can take in the game, and make sure that this code is not inside the loop as it would be very annoying to have to read it over and over again.

3. Game Loop

``` rust
loop {
    // Game code...
}
```
The `loop` keyword in Rust is used to create an infinite loop. This is the main game loop where all the actions take place.

4. display status

``` rust
println!("current energy: {}\nmax energy: {}\nstrength: {}\nmoney: {}\n",energy,max_energy,strength,money); // displays user stats
```
this is crucial for making the game  playable, without knowing your stats or having any way to find them makes the game unplayable, as a bonus you could make the code only print your stats when you want it too, The `{}` are placeholders that get replaced by the variables provided after the string.

<details>
  <summary>how to make it print stats when stat is typed</summary>

  you can print the stats when typed by just adding an if statement in the loop too detect if the user types stats, I will go over the handling user input part later on.
  
  ```rust
  if input == "stats" {
    println!("current energy: {}\nmax energy: {}\nstrength: {}\nmoney: {}\n",energy,max_energy,strength,money);
  }
  ```

</details>

5. User Input

```rust
let mut input = String::new();
io::stdin().read_line(&mut input)
    .expect("Failed to read line input.");
let input = input.trim().to_string();
```

The read_line function is used to get user input from the console. It reads a line from the standard input and stores it in the input variable (make sure you include the `std::io` library at the top of your code, you can do this with: `use std::io;`.

6. handling user actions.

   Depending on the user's input, different actions are performed. If the user types "train", they are asked to input a weight, and if that weight is less than their energy, their energy is reduced by the weight and their strength increases by the weight. If the user types "bench", they have the option to regain full energy at the cost of a quarter of their strength. If the user types "shop", they can convert half their strength into max energy. If the user types "flex", they lose half their energy and gain money proportional to their strength.

The following is an example of two of the actions:
```rust 
if input == "train" {
    println!("--input weight--");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight)
        .expect("Failed to read line input.\n");
    let weight: i128 = weight.trim().parse().unwrap();
    if weight >= energy {
        println!("\n***failed to bench***\n");
    }
    if weight < energy {
        energy = energy - weight;
        strength = strength + weight;
        println!("Weight benched succesfully");
    }
} else if input == "bench" {
            println!("\nJerald: welcome to the bench, would you like a drink, 'yes' or 'no'? (+energy, -fourth of strength)");
            let mut option = String::new();
            io::stdin().read_line(&mut option)
                .expect("Failed to read line input.");
            let option = option.trim().to_string();
                if option == "yes" {
                    println!("*you drank the drink and regained full energy, but you lost {} strength*\n",strength/4);
                    strength = strength - strength/4;
                    energy = max_energy;
                    if strength < 0 {
                        strength = 0;
                    }
} 

```

the block checks if the user inputs train, and then handles another input inside of it, and then if the user doesn't input that it checks if they input bench, and then has basically the same thing going on (you can name your other input variable different, you don't have too but I advise it just because I like doing it).

This loop continues indefinitely until the program is manually stopped, allowing the user to continue performing actions as long as they want.
This is a basic interactive program in Rust that demonstrates the use of variables, loops, conditionals, and basic input/output. It's a good starting point for learning about game loops and text-based user interaction in Rust.

WHEN CHANGING VARIABLES IN THE LOOP DO NOT USE `const` or `let` instead refer to them like I do in the code `energy = max_energy;`.








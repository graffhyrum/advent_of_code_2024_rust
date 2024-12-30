# Advent of Code 2024

## Setup

1. **Clone the repository:**
   ```sh
   git clone https://github.com/graffhyrum/advent_of_code_2024_rust.git
   cd advent_of_code_2024_rust
   ```

2. **Install Rust:**
   If you don't have Rust installed, you can install it using [rustup](https://rustup.rs/):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Build the project:**
   ```sh
   cargo build
   ```

## Usage

1. **Run the project:**
   ```sh
   cargo run -- [day]
   ```
   Replace `[day]` with the day number you want to run (e.g., `cargo run -- 1`).

2. **Input files:**
   Ensure that the input files are placed in the `src/inputs` directory and named as `day_[day].txt` (e.g., `src/inputs/day_1.txt`).

## Example

To run the solution for day 1:
```sh
cargo run -- 1
```

## Project Structure

- `src/main.rs`: Entry point of the application.
- `src/days/`: Contains the implementation for each day's problem.
- `src/inputs/`: Directory for input files.
- `src/problems.rs`: Defines the `Problem` trait.

## Adding a New Day

1. **Create a new file for the day:**
   ```sh
   touch src/days/day_[day].rs
   ```
   Replace `[day]` with the new day number.

2. **Implement the `Problem` trait:**
   ```rust
   use crate::problems::Problem;

   pub struct Day[Day];

   impl Problem for Day[Day] {
       fn part_one(&self, input: &str) -> String {
           // Your implementation here
       }

       fn part_two(&self, input: &str) -> String {
           // Your implementation here
       }
   }
   ```

3. **Update `src/days/mod.rs`:**
   ```rust
   pub mod day_[day];
   pub use day_[day]::Day[Day];
   ```

4. **Update `day_to_problem` function in `src/main.rs`:**
   ```rust
   26 => Some(Box::new(days::Day[Day])),
   ```

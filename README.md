# Simple Calculator Project Specification

## Project Overview

The goal of this project is to create a simple calculator that can perform basic arithmetic operations such as addition, subtraction, multiplication, and division. The calculator will be a command-line application written in Rust.

## Requirements

### Functional Requirements

1. **Addition**: The calculator should be able to add two numbers.
2. **Subtraction**: The calculator should be able to subtract one number from another.
3. **Multiplication**: The calculator should be able to multiply two numbers.
4. **Division**: The calculator should be able to divide one number by another, with proper handling of division by zero.
5. **Input Handling**: The calculator should accept user input for the operation and numbers.
6. **Output**: The result of the calculation should be displayed to the user.

### Non-Functional Requirements

1. **Performance**: The calculator should perform operations quickly and efficiently.
2. **Usability**: The calculator should have a simple and intuitive command-line interface.
3. **Reliability**: The calculator should handle errors gracefully, such as invalid input or division by zero.

## Technical Specification

### Language

- The application will be written in Rust.

### Dependencies

- No external dependencies are required for this project.

### Modules

- **main.rs**: The entry point of the application.
- **calculator.rs**: Contains the implementation of the calculator functionalities.

### Functions

#### `main`

- Description: The entry point of the application. Handles user input and calls the appropriate calculator functions.
- Signature: `fn main()`

#### `add`

- Description: Adds two numbers.
- Signature: `fn add(a: f64, b: f64) -> f64`

#### `subtract`

- Description: Subtracts the second number from the first number.
- Signature: `fn subtract(a: f64, b: f64) -> f64`

#### `multiply`

- Description: Multiplies two numbers.
- Signature: `fn multiply(a: f64, b: f64) -> f64`

#### `divide`

- Description: Divides the first number by the second number. Returns an error message if division by zero is attempted.
- Signature: `fn divide(a: f64, b: f64) -> Result<f64, &'static str>`

## User Interaction

1. The user runs the program.
2. The program prompts the user to enter the first number.
3. The user enters the first number.
4. The program prompts the user to enter the operation (+, -, \*, /).
5. The user enters the operation.
6. The program prompts the user to enter the second number.
7. The user enters the second number.
8. The program performs the calculation and displays the result.
9. The program prompts the user if they want to perform another calculation or exit.

## Error Handling

- If the user enters an invalid number, the program should display an error message and prompt the user to enter the number again.
- If the user attempts to divide by zero, the program should display an error message and prompt the user to enter a valid divisor.

## Example Usage

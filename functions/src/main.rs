fn main() {
    println!("Hello, world!");

    /* Functions without return values */
    
    another_function();

    parametrised_function(5, 6);

    /* Statements and Expressions
     * - statements are instructions that perform some action and do not return
     *   a value.
     * - expressions evaluate to a value.
     */

    let x = 6; // statement, i.e. `let y = (let x = 6);` will result in error.

    // {} block creates a new scope, it is an expression.
    let y = {
        let x = 3;
        x + 1 // expression that is returned. Adding semicolumn => statement.
    };

    println!("The value of y is: {}", y);

    // Macros are expressions. Also see the demostration of different scopes.
    let _z = println!("The value of x in outer scope is: {}", x); // x is not 3.

    /* Functions with return values - obviously an expression*/

    let x = five(); // Identical to `let x = 5`.
    println!("The value of x is: {}", x);

    let y = three(); // Identical to `let y = 3`.
    println!("The value of y is: {}", y);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
    

}

// Position of function definition is irrelevant. Definition is a statement.
fn another_function() { // Naming convention is to use snake_case (like vars).
    println!("Another function.");
}

fn parametrised_function(x: i32, y: i32) { // Type must be declared.
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
}

// An function with a return value
fn five() -> i32 { // Return type must be explicit
    1;
    2;
    3;
    4;
    5 // return value is the value of tthe final expression in the block
}

fn three() -> i32 {
    1;
    2;
    return 3; // return keyword to specify a value early and explicitely
    4; // Unreachable warning expected. `return` is used when branching occurs.
    5 
}

// An function with a parameter and a return value
fn plus_one(x: i32) -> i32 {
    x + 1 // Adding a semicolon=> statement => no return= >breaking def.=> err.
}

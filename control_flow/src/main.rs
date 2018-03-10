fn main() {
    /* `if` expression */

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Condition must be bool.
    let x = true;
    if x { // If `let x = 5` this woudl raise mismatched types error.
        println!("condition was true");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    /* Multiple conditions
     * - too many `else if` statements can clutter the code; use `match`.
     */

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /* `if` is an expression 
     * - {} block evaluate to a value; blocks are chosen based on the condition.
     */

    let condition = true;

    let number = if condition { 
        5
    } else {
        6 // Must be same type as 5; "six" would not work. Types must be known
          // at compile type, as a safety measure.
    };

    println!("The value of number is: {}", number);

    /* Repetition with loops
     * - three kinds: `loop`, `while` and `for`.
     */

    // `loop` - loops forever
    
    loop {
        println!("again!");
        break; // Use break to halt the loop. This works for any kind of loop.
    }

    // `while` - conditional loop

    let mut number = 3;

    while number != 0 { // Repeat while number is non zero.
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // `for` - looping through a collection

    // Looping through a collection with `while` => index hell, slow.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { // A condition must be checked => slow. Length must be OK.
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    // Streamlined approach using `for`.
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // `for` loop are more concise and safer even if the number of iterations is
    // hardcoded. This is done using the `Range` standard lib type.condition
    for num in (1..4).rev() { // note that `num` is not explicitely declared.
        println!("the value is: {}", num);
    }
    println!("LIFTOFF!!!");

}

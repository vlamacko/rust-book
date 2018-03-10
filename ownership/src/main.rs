/* Ownership
 * - memory management system; set of rules that can be checked at compile time.
 *      - no run-time cost, i.e. garbage collection.
 *      - safety ensured.
 */

// N.B.: Read the explanation of stack and heap carefully.

/* Rules - simple but impactful.
 *  1. Each value in Rust has a variable that is called its owner.
 *  2. There can only be one owner at a time.    
 *  3. When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    /* Scope */
    
    // Introduce a new block/scope for demonstration purposes. The body of any 
    // function (e.g. `main`) is just another block/scope. 
                       
    {                       // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward

        println!("{}", s);
    };                      // this scope is now over, and s is no longer valid
                            // `println!("{}", s);` would throw an error.

    /* The `String` type
     * - not a primitive type, stored on heap instead. Previous types were all
     *   stored on the stack.
     * - a primitive string literal `let s = "hello"
     *      - hardcoded and immutable, stored on stack.
     *      - its value must be known at compile time, i.e. when writing code.
     * - can be mutated.
     */

    let mut s = String::from("hello"); // Created from string literal. 

    s.push_str(", world!"); // Appends a literal => mutaliblity.

    println!("{}", s);

    /* Memory and Allocation */

    {
        let s = String::from("hello"); // Memory on the heap is requested here at
                                    // runtime. s is valid from this point.

        println!("{}", s);
    }                              // This scope is now over, and s is no
                                   // longer valid. Memory is freed using `drop`
                                   // function defined for the `String` type.
    
    // Moving variables

    let x = 5;
    let y = x; // Copies the value of x. There are two integers 5 on the stack.

    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1; // Copies the pointer on the stack to the string on heap. Rust 
                 // then actually frees the s1 pointer. This is to prevent a 
                 // double free error since s1 and s2 leave the scope at the
                 // same time. Thus this action is called move.
    
                 // `println!("{}, world!", s1);` would raise an error.
    println!("s2 = {}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Cloning == deep copy. Now there are two pointers
                         // refering to two strings on the heap. This is to 
                         // prevent double free error. It may be costly.

    println!("s1 = {}, s2 = {}", s1, s2); // s1 has not been freed.

    // Stack only data: Copy

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // x is still accessible; stack only.
                                      // This means i32 has a `Copy` trait, heap
                                      // based types must have `Drop` trait.

    /* Copy traits
     * - scalar types generally always have the `Copy` trait.
     * - compoud types like tupes only if they consist of purely `Copy` types.
     */

    /* Ownership and Functions */

    outer_function(); // Demonstraiting the difference `Copy` and `Drop` types.
                      // when passed into the function.

    another_outer_function(); // Demonstrating usage of funciton returns to
                              // pass aroung ownership.

    yet_another_outer_funciton(); // Demonstrating using tuples to take and give
                                  // give back while also supplying additional 
                                  // data.
}


/* Inner functions with no return values */

fn outer_function() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

// Function with no return value.
fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);

} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

// Function with no return value.
fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);

} // Here, some_integer goes out of scope. Nothing special happens.


/* Inner functions with return values */

fn another_outer_function() {
    let _s1 = gives_ownership();        // gives_ownership moves its return
                                        // value into s1.

    let s2 = String::from("hello");     // s2 comes into scope.

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

// gives_ownership will give a new ownership to the outer scope --- calling fun.
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope.

    a_string  // a_string is returned and moves out to the calling function.
}


/* Inner functions return a tuple */

fn yet_another_outer_funciton() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // The original s1 is returned back in
                                          // in form of s2 ownership with
                                          // additional data returned in form of
                                          // len variable.

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
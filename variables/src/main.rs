fn main() {
    /* Mutability */

    let mut x = 5; // x must be mutable in order to change its value.
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /* Constatnts
     * - always immuatable.
     * - must be set to a constant expression (known before runtime).
     * - can be declared in global scope.
     */

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    /* Shadowing
     * - same as Python (except mutable object like list) but instead shadowing
     *   is explicit.
     * - it is possible to change the type of the variable this way by reusing
     *   the name.
     */

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

}

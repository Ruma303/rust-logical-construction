fn main() {
    // Expressions

    let x = 5;
    println!("x is {}", x);

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Questa espressione verrà assegnata a `y`
        x_cube + x_squared + x
    };
    println!("y is {:?}", y);

    let z = {
        // Il punto e virgola sopprime questa espressione, e `()` viene assegnato a `z`
        2 * x;
    };
    println!("z is {:?}", z);

    // Statements

    let a = 10; // Statement: dichiarazione e assegnazione di variabile
    let b = false; // Statement
    println!("Hello, World!"); // Statement

    // Constructors

    // Questo è un blocco, contiene una serie di statements
    {
        let x = 5; // Statement all'interno del blocco
        println!("x = {}", x); // Statement
    } // Fine del blocco
}

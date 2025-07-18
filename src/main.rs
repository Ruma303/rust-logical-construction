fn main() {

    // Operatori aritmetici
    let a = 10;
    let b = 3;

    println!("Addizione: {}", a + b);
    println!("Sottrazione: {}", a - b);
    println!("Moltiplicazione: {}", a * b);
    println!("Divisione: {}", a / b);
    println!("Modulo: {}", a % b);



    // Operatori di assegnazione
    let mut x = 10;
    x += 5; // x = x + 5
    println!("Dopo +=: {}", x);

    x *= 2; // x = x * 2
    println!("Dopo *=: {}", x);

    x /= 3; // x = x / 3
    println!("Dopo /=: {}", x);

    x -= 1; // x = x - 1
    println!("Dopo -=: {}", x);

    x %= 2; // x = x % 2
    println!("Dopo %=: {}", x);



    // Operatori di confronto
    let a = 10;
    let b = 20;

    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a <= b: {}", a <= b);



    // Operatori logici
    let x = true;
    let y = false;

    println!("x && y: {}", x && y);
    println!("x || y: {}", x || y);
    println!("!x: {}", !x);



    // Operatori bitwise
    let a = 5; // 0b0101
    let b = 3; // 0b0011

    println!("AND bitwise: {}", a & b);
    println!("OR bitwise: {}", a | b);
    println!("XOR bitwise: {}", a ^ b);
    println!("Shift sinistra: {}", a << 1);
    println!("Shift destra: {}", a >> 1);



    // Altri operatori
    let range = 1..5; // Include 1, 2, 3, 4 ma non 5
    for i in range {
        println!("{}", i);
    }

    for i in 1..=10 {
        println!("{}", i);
    }

    let x = Some(5);

    match x {
        Some(n @ 1..=9) => println!("Numero piccolo: {}", n),
        Some(_) => println!("Altro numero"),
        None => println!("Nessun valore"),
    }

    let none = Option::<i32>::None;

    match none {
        Some(n @ 1..=9) => println!("Numero piccolo: {}", n),
        Some(_) => println!("Altro numero"),
        None => println!("Nessun valore"),
    }

    fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n = s.parse::<i32>()?; // se fallisce, restituisce l'errore
        Ok(n)
    }

    let n = parse_int("123").unwrap();
    let m = parse_int("abc").unwrap_err();

    println!("n: {}", n);
    println!("m: {}", m);



    // Operatori di ownership e borrowing
    let mut x = 10;
    let y = &x; // Borrowing immutabile
    println!("y: {}", y);

    let z = &mut x; // Borrowing mutabile
    *z += 5; // Dereferenziazione
    println!("x dopo mutazione: {}", x);

    let mut data = 42;
    {
        let r = &data; // borrow immutabile
        println!("r: {}", r);
    } // fine del borrow immutabile

    let m = &mut data;
    *m += 1;
    println!("data: {}", data);

    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() > b.len() { a } else { b }
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);


    // Operatore di conversione
    let x = 10;
    let y = x as f64; // Conversione da i32 a f64
    println!("y: {}", y);

    let x: f64 = 65.4321;
    let y = x as u8; // Tronca a 65
    println!("y: {}", y);


    // Operatori di tipo

    fn print_vec<T: std::fmt::Debug>(v: Vec<T>) {
      println!("{:?}", v);
    }

    let v = Vec::<i32>::new(); // specifica esplicita
    print_vec(v);


    // Operatori ternari
    let x = 10;
    let y = if x > 5 {
        "x è maggiore di 5"
    } else {
        "x è minore o uguale a 5"
    };
    println!("{}", y);
}

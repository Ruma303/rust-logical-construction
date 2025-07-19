enum Status {
    Found,
    NotFound,
}

struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    // Blocchi condizionali
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    // Espressioni condizionali
    let x = 10;
    let big_x = if x < 10 && x > -20 {
        println!("{} is small", x);
        10 * 2
    } else if x == 10 {
        println!("{} is 10", x);
        10
    } else {
        println!("{} is big", x);
        10
    };

    println!("{} -> {}", x, big_x);

    // Operatore ternario
    let x = 5;
    let y = if x > 5 { 10 } else { 20 };
    println!("{} -> {}", x, y);

    let number = Some(42);
    let result = if number.is_some() {
        "Valore trovato"
    } else {
        "Nessun valore"
    };

    println!("{}", result); // Output: Valore trovato

    // Espressioni avanzate

    // if let
    let status = Some(42);
    if let Some(value) = status {
        println!("Valore trovato: {}", value);
    } else {
        println!("Nessun valore trovato");
    }

    // match
    let status = Status::Found;
    let message = match status {
        Status::Found => "Valore trovato",
        Status::NotFound => "Nessun valore trovato",
    };
    println!("{}", message);

    // Destrutturazione tuple tramite pattern matching
    let point = (3, 0);
    match point {
        (0, y) => println!("Il punto si trova sull'asse y: {}", y),
        (x, 0) => println!("Il punto si trova sull'asse x: {}", x),
        (x, y) => println!("Il punto è nelle coordinate: ({}, {})", x, y),
    }

    // Destrutturazione struct tramite pattern matching
    let punto = Punto { x: 0, y: 5 };

    match punto {
        Punto { x: 0, y } => println!("Il punto si trova sull'asse y a {}", y),
        Punto { x, y: 0 } => println!("Il punto si trova sull'asse x a {}", x),
        Punto { x, y } => println!("Il punto è a coordinata ({}, {})", x, y),
    }

    // Destrutturazione con pattern di referenza
    let valore = 42;
    let riferimento = &valore;

    match riferimento {
        &val => println!("Il valore è: {}", val),
    }

    // Destrutturzione con riferimento mutabile
    let valore = 70;

    match valore {
        ref val => println!("Il valore é: {}", val),
    }

    // Destrutturazione pattern complessi
    let valore = Some((10, 20));

    match valore {
        Some((x, y)) => println!("Valori nella tupla: {}, {}", x, y),
        None => println!("Nessun valore"),
    }

    // Quando il pattern matching non è necessario
    let risultato = divisione(9.0,4.0);
    match risultato {
        Some(numero) => {println!("Valore trovato: {}", numero)},
        None => {}
    }

    // Alternativa con if let
    if let Some(numero) = divisione(9.0,4.0) {
       println!("Risultato ottenuto: {}", numero);
    }

    // Alternativa con let else
    let Some(numero) = divisione(9.0, 0.0) else {
      panic!("Divisione per zero non consentita!");
    };

    println!("Risultato ottenuto: {}", numero);
}

fn divisione(dividendo: f32, divisore: f32) -> Option<f32> {
    if divisore != 0.0 { Some(dividendo / divisore) } else { None }
}
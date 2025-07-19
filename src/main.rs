fn main() {
    // while
    let mut count = 0;
    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    // breake e continue
    println!("\nBreak e Continue");
    let mut count = 1;
    while count < 10 {
        if count == 3 {
            println!("Operazione saltata con valore count = {}", count);
            count += 1;
            continue;
        }

        if count == 5 {
            println!("Operazione interrotta al valore count = {}", count);
            break;
        }

        println!("count: {}", count);
        count += 1;
    }

    // for in
    for num in 1..5 {
        println!("num: {}", num);
    }

    for num in 1..=5 {
        println!("num: {}", num);
    }

    for num in (1..5).rev() {
        if num == 2 {
            println!("Operazione saltata con valore num = {}", num);
            continue;
        }

        println!("num: {}", num);
    }

    // Ignorare il valore dell'iterazione
    for _ in 0..3 {
        println!("Ripetizione senza contatore");
    }

    // Iterare sulle collezioni
    let nomi = ["Luca", "Marco", "Sara"];

    for nome in nomi.iter() {
        println!("Ciao, {}", nome);
    }

    // loop
    let mut contatore = 1;
    loop {
        contatore += 1;
        if contatore % 2 == 0 {
            continue;
        } else {
            println!("... {}", contatore);
        }
        if contatore > 10 {
            break;
        }
    }

    // Esempio di loop che ritorna un valore
    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break count;
        }
    };

    println!("Result: {}", result);

    // Nested loops e labels
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // Esempio avanzato di nested labeled loops
    println!("\nnested labeled loops:");
    let mut count = 0;
    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Esempio avanzato con continue
    println!("\nnested labeled loops with continue:");
    let mut count = 0;

    let result: i32 = 'outer: loop {
        println!("Entered the outer loop (count = {})", count);

        'inner: loop {
            println!("Entered the inner loop");

            if count == 3 {
                println!("Uscita dal ciclo con count = {}", count);
                // Corretto: break con etichetta verso 'outer
                break 'outer count;
            }

            count += 1;

            // Salta il ciclo interno e riprende da 'outer
            continue 'outer;
        }

        // Questo non verrà mai eseguito in questo esempio
        println!("Fine del ciclo interno");
        break 'outer 999;
    };

    println!("Exited the outer loop with result = {}", result);

    fizzbuzz();
}

fn fizzbuzz() {
  let mut fizz_buzz: i8 = 0;
	let mut fizz: i8 = 0;
	let mut buzz: i8 = 0;

	for number in 1..=100 {
		if number % 3 == 0 && number % 5 == 0 {
			println!("FizzBuzz");
			fizz_buzz += 1;
		} else if number % 3 == 0 {
			println!("Fizz");
			fizz += 1;
		} else if number % 5 == 0 {
			println!("FizzBuzz");
			buzz += 1;
		}
	}

	println!("\nDa 0 a 100 ci sono totale di {} FizzBuzz, {} Fizz e {} Buzz", fizz_buzz, fizz, buzz);
}

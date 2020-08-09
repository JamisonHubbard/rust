use std::io;

fn main() {
    println!("Enter the value of n: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    
    let n: i64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let f = fibonacci(n);

    if f < 0 {
        println!("Not a valid input of n");
    }
    else {
        println!("The n={} fibonnaci value is: {}", n, f);
    }

    println!("\nEnter a sentence: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Index of the word you want: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let word_at_n = nth_word(&n, input.as_str());

    println!("The word at index {} is: {}", n, word_at_n);
}

fn fibonacci(mut n: i64) -> i64 {
    // negative n is invalid
    if n < 0 {
        return -1;
    }
    // by definition returns 1
    else if n <= 2 {
        return 1;
    }

    let mut a: i64 = 1;
    let mut b: i64 = 1;

    while n > 2 {
        let f = a + b;
        a = b;
        b = f;
        n -= 1;
    }

    b
}

fn nth_word<'a>(n: &'a usize, s: &'a str) -> &'a str {
    let mut count = 0;
    let mut prev_start = 0;
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            if count == *n {
                return &s[prev_start..i];
            }

            prev_start = i + 1;
            count += 1;
        }
    }

    if count == *n {
        &s[prev_start..]
    }
    else {
        "Index too large"
    }
}
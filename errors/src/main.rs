use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let _trash = index_panic(false);
    file_read_error(false);
    unwrap_file_read_error(false);
    println!("0 errors, 0 panics");
}

fn index_panic(b: bool) -> i32 {
    let v = vec![1, 2, 3];

    if b {
        return v[99];
    }
    
    v[2]
}

fn file_read_error(b: bool) {
    if !b {return;}

    let f = File::open("src/hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(_error) => panic!("Problem opening the file: {:?}", _error)
    };

    println!("File opened: {:?}", f);
}

fn unwrap_file_read_error(b: bool) {
    if !b {return;}

    let _f = File::open("src/hello.txt").unwrap();

    // f will now be the contents of the Ok(content) if the file read is
    // succesful, or will automatically call panic! thanks to unwrap()
}

fn _read_from_file_long_way() -> Result<String, io::Error> {
    // f is a Result type
    let f = File::open("hello.txt");

    // if f is an error it returns the error
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    // if f can be read to a string it is and is returned
    // if it can't and it errors then the error is returned
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn _read_from_file_medium_way() -> Result<String, io::Error> {
    // the ? operator does almost the same thing as the match blocks above
    // if its expression errors, it calls std::From::from() which converts
    // the error type to whatever error type is being returned and then
    // returns it, only if return type is Result or Option or some other 
    // type that implements std::ops::Try

    // f is either a file reference or returns an error
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    // s either reads in f as a String or returns an error
    f.read_to_string(&mut s)?;
    // returns read data
    Ok(s)
}

fn _read_from_file_short_way() -> Result<String, io::Error> {
    let mut s = String::new();
    
    // the single ? operator can be used multiples times
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn _read_from_file_shortest_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
fn main() {
    let v1 = vec![1, 2, 3];

    for v in v1.iter() {
        print!("{}  ", v);
    }
    println!();

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    for v in v2.iter() {
        print!("{}  ", v);
    }
    println!();
}

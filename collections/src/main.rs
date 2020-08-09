

fn main() {
    vector_practice();
    string_practice();
    hash_map_practice();
}

fn vector_practice() {
    // type required
    let mut v1: Vec<i32> = Vec::new();
    
    let mut i = 5;
    while i > 0 {
        v1.push(i);
        i -= 1;
    }

    // type implied
    let v2 = vec![1, 2, 3, 4, 5];

    println!("Vec1: {:?}\nVec2: {:?}", v1, v2);
    println!("Vec1 length: {}\nVec2 length: {}", v1.len(), v2.len());
}

fn string_practice() {
    let t1 = "tic";
    let t2 = "tac";

    let s1 = String::from(t1);
    let s2 = t2.to_string();
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    let russian = "Здравствуйте";
    let devanagari = "न";

    // these panic if false
    assert_eq!(&t1[0..1], "t");         // each char is 1 byte
    assert_eq!(&russian[0..4], "Зд");   // each char is 2 bytes
    assert_eq!(&devanagari[0..3], "न"); // each char is 3 bytes

    for c in t1.chars() {
        print!("{} ", c);
    }
    println!();

    for b in t1.bytes() {
        print!("{} ", b);
    }
    println!();
}

fn hash_map_practice() {
    use std::collections::HashMap;

    // HashMaps by insertion
    let mut scores_one = HashMap::new();

    scores_one.insert(String::from("Blue"), 10);
    scores_one.insert(String::from("Yellow"), 50);

    println!("Hashmap 1: {:?}", scores_one);

    // Hashmaps by collection
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores_two: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Hashmap 2: {:?}", scores_two);
}
use std::fmt;

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IP Address: {}", home);
    println!("Loop IP Address: {}", loopback);
}

#[derive(Debug)]
enum IpAddr {
    V4(String), V6(String)
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddr::V4(val) => write!(f, "{} (v4)", val),
            IpAddr::V6(val) => write!(f, "{} (v6)", val)
        }
    }
}
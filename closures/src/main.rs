use std::thread;
use std::time::Duration;

fn main() {
    let sim_user_val = 10;
    let sim_random = 7;

    generate_workout_expensive(sim_user_val, sim_random);
    println!("\n");
    generate_workout_almost_functional(sim_user_val, sim_random);
}

fn sim_calculation(intensity: u32) -> u32 {
    let calc_time = 1;

    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(calc_time));
    intensity
}

fn generate_workout_expensive(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today do {} pushups!", sim_calculation(intensity));
        println!("Next, do {} situps!", sim_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to hydrate!");
        } else {
            println!("Today, run for {} minutes", sim_calculation(intensity));
        }
    }
}

fn generate_workout_almost_functional(intensity: u32, random_number: u32) {
    let expensive_result = sim_calculation(intensity);
    
    if intensity < 25 {
        println!("Today do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to hydrate!");
        } else {
            println!("Today, run for {} minutes", expensive_result);
        }
    }
}

fn _generate_workout_closure(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        let calc_time = 1;
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(calc_time));
        num
    };
    
    if intensity < 25 {
        println!("Today do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to hydrate!");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity));
        }
    }
}

// Calc_Cacher can hold a closure and a value stored in an Option
// if the value is Option::None and the value() function is called
// the struct calls the closure to generate the closure
// if the value has already been calculated it simply returns that
struct _Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> _Cacher<T> where T: Fn(u32) -> u32 {
    fn _new(calculation: T) -> _Cacher<T> {
        _Cacher {
            calculation,
            value: None
        }
    }

    fn _value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn _generate_workout_closure_struct(intensity: u32, random_number: u32) {
    // define Calc_Cacher struct with the included closure as the <T> generic
    let mut expensive_closure = _Cacher::_new(|num| {
        let calc_time = 1;
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(calc_time));
        num
    });

    if intensity < 25 {
        println!("Today do {} pushups!", expensive_closure._value(intensity));
        println!("Next, do {} situps!", expensive_closure._value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to hydrate!");
        } else {
            println!("Today, run for {} minutes", expensive_closure._value(intensity));
        }
    }
}
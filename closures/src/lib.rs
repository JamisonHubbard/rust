use std::collections::HashMap;

// Improved version of the Cache struct
// Uses generics to take a closure with any parameter and 
// return types
// Uses a hashmap to only complete an expensive calculation
// when it hasn't already been done for that input

/// Stores a closure with generic parameter A and generic return
/// type B. Also stores a HashMap of previously calculated results
/// of the closure
/// 
/// # Examples
/// 
/// ```
/// let mut my_cache = lib::Cache::new(|&x| x);
/// 
/// let a = my_cache.value(5);  // calculates using closure
/// let b = my_cache.value(5);  // pulls from HashMap
/// 
/// assert_eq!(a, b);
/// ```
pub struct Cache<A, B, T> where 
    T: Fn(&A) -> B,
    A: std::cmp::Eq + std::hash::Hash,
    B: Copy
{
    calculation: T,
    values: HashMap<A, B>
}

impl<A, B, T> Cache<A, B, T> where 
    T: Fn(&A) -> B,
    A: std::cmp::Eq + std::hash::Hash,
    B: Copy
{
    /// Returns a new instance of the Cache struct
    /// 
    /// Examples
    /// ```
    /// let my_cache = lib::Cache::new(|x| x + 1);
    /// ```
    pub fn new(calculation: T) -> Cache<A, B, T> {
        let new_map: HashMap<A, B> = HashMap::new();
        
        Cache {
            calculation,
            values: new_map
        }
    }

    /// Returns the result of the Cache's closure with 
    /// the given parameter. 
    /// 
    /// If the value has already been calculated it is
    /// pulled from a HashMap, otherwise the closure is
    /// called.
    /// 
    /// Examples
    /// ```
    /// let mut my_cache = lib::Cache::new(|x| x + 1);
    /// let my_value = my_cache.value(5);
    /// 
    /// assert_eq!(my_value, 6);
    /// ```
    pub fn value(&mut self, arg: A) -> B {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use super::*;

    #[test]
    fn cache_same_param() {
        let mut cache_one = Cache::new(|&num| {
            let calc_time = 1;
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(calc_time));
            num
        });

        let a = cache_one.value(1);
        let b = cache_one.value(1);

        assert_eq!(a, b);
    }
    
    #[test]
    fn cache_diff_param() {
        let mut cache_one = Cache::new(|&num| {
            let calc_time = 1;
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(calc_time));
            num
        });

        let a = cache_one.value(1);
        let b = cache_one.value(2);

        assert_ne!(a, b);
    }

    #[test]
    fn cache_diff_types() {
        let mut cache_one = Cache::new(|&num| {
            let calc_time = 1;
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(calc_time));
            num
        });

        let a1 = cache_one.value(1);
        let b1 = cache_one.value(2);

        let mut cache_two = Cache::new(|string: &&str| {
            let size = string.len();
            size
        });

        let a2 = cache_two.value("dog");
        let b2 = cache_two.value("puppy");

        assert_eq!(a1, 1);
        assert_eq!(b1, 2);

        assert_eq!(a2, 3);
        assert_eq!(b2, 5);
    }
}
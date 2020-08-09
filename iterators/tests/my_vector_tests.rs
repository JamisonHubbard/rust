mod my_vector_tests {
    use lib::my_vector::*;

    #[test]
    fn push_and_pop() {
        let mut vec1 = MyVec::new();

        vec1.push(1);
        vec1.push(1);
        vec1.push(7);

        let c = vec1.pop().unwrap();
        let b = vec1.pop().unwrap();
        let a = vec1.pop().unwrap();

        let result = String::from(
            format!("{} {} {}", a, b, c)
        );

        assert_eq!(result, String::from("1 1 7"));
    }

    #[test]
    fn basic_iter() {
        let mut vec1 = MyVec::new();

        vec1.push(1);
        vec1.push(1);
        vec1.push(7);

        let mut v_iter = vec1.iter();
        let mut result = String::new();
        
        while true {
            match v_iter.next() {
                Some(val) => {
                    let add = String::from(
                        format!("{} ", val)
                    );

                    result.push_str(&add);
                },
                None => break
            }
        }

        assert_eq!(result, String::from("1 1 7 "));
    }

    #[test]
    fn sum_adaptor() {
        let mut vec1 = MyVec::new();

        vec1.push(1);
        vec1.push(1);
        vec1.push(7);

        let total: i32 = vec1.iter().sum();

        assert_eq!(total, 9);
    }

    #[test]
    fn map_adaptor() {
        let mut vec1 = MyVec::new();

        vec1.push(1);
        vec1.push(1);
        vec1.push(7);

        let vec2: Vec<_> = vec1.iter()
            .map(|x| x + 1)
            .collect();

        let total: i32 = vec2.iter().sum();

        assert_eq!(total, 12);
    }
}
// a rough implementation of a vector
// that cheats by using Vec internally
// but includes its iterator
// which implements the iterator trait
// which is the whole point

use std::iter::Iterator;
use std::convert::TryInto;

pub struct MyVec<T: Copy> {
    data: Vec<T>,
    size: u32
}

pub struct MyVecIter<T: Copy> {
    index: u32,
    vector: MyVec<T>
}

impl<T> MyVec<T> where T: Copy{
    pub fn new() -> MyVec<T> {
        let new_data = Vec::new();

        MyVec {
            data: new_data,
            size: 0
        }
    }

    pub fn get(&self, index: u32) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let index: usize = index.try_into().unwrap();

        Some(self.data[index])
    }

    pub fn push(&mut self, item: T) {
        self.size += 1;
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size != 0 {
            self.size -= 1;
        }
        self.data.pop()
    }

    pub fn iter(self) -> MyVecIter<T> {
        MyVecIter {
            index: 0,
            vector: self
        }
    }
}

impl<T> Iterator for MyVecIter<T> where T: Copy{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.vector.get(self.index);
        self.index += 1;
        result
    }
}
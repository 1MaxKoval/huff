use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::cmp::{Ord, Ordering};
use std::fs::File;
use std::io::prelude::*;

struct HuffCodeString {
    data: Vec<u8>,
    bit_pointer: u8,
    bits_set: u8,
    elem_pointer: usize,
}

impl HuffCodeString {

    fn new() -> Self {
        return HuffCodeString {
            data: vec![0],
            bit_pointer: 128, // 10000000
            elem_pointer: 0,
            bits_set: 0
        };
    }

    fn set_bit(&mut self, b: bool) {
        if b {
            let mut v = self.data[self.elem_pointer];
            self.data[self.elem_pointer] = v | self.bit_pointer
        }
        if self.bit_pointer == 1 { // Reached 0000001
            self.bit_pointer = 128;
            self.elem_pointer += 1;
            self.data.push(0);
        }
        else { self.bit_pointer = self.bit_pointer >> 1; }
    }

    fn consume(code_string: HuffCodeString) {

    }

}

impl Iterator for HuffCodeString {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
    }

}

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    weight: usize,
    data: Option<T>
}

impl<T> Ord for Node<T> {

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }

}

impl<T> PartialOrd for Node<T> {
    
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        if self.weight < other.weight {
            return Some(Ordering::Less);
        }
        else if self.weight > other.weight {
            return Some(Ordering::Greater);
        }
        else { return Some(Ordering::Equal) }
        
    }

}

impl<T> PartialEq for Node<T> {

    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }

}

impl<T> Eq for Node<T> {}

fn write_bytes(data: &Vec<u8>, file_path: &str) {
    let mut f = File::create(file_path)
        .expect("Couldnt read create a file");
    f.write_all(data).expect("Couldnt write to file");
}

fn get_coding_scheme(head_node: Node<char>, data: &str) -> HashMap<char, usize> {

}

fn construct_huffman_tree(f: &HashMap<char, usize>) -> Node<char> {
    let mut heap = BinaryHeap::new();
    f.iter()
        .for_each(|(c, i)| { heap.push(
            Reverse(
                Node {
                    left: Option::None,
                    right: Option::None,
                    data: Some(c.clone()),
                    weight: i.clone()
                }
            )
        ) });
    for _ in 0..heap.len() {
        let Reverse(first_node) = heap.pop().unwrap();
        let Reverse(second_node) = heap.pop().unwrap();
        let sum = first_node.weight + second_node.weight;
        heap.push(
            Reverse(
                Node {
                    left: Some(Box::new(first_node)),
                    right: Some(Box::new(second_node)),
                    data: Option::None,
                    weight: sum
                }
            )
        )
    }
    let Reverse(node) = heap.pop().unwrap();
    node
}

fn main() {
}

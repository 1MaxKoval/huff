use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::cmp::{Ord, Ordering};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct BitString {
    data: Vec<u8>,
    bit_pointer: u8,
    bits_set: u8,
    elem_pointer: usize,
    iter_elem_p: usize
}

impl BitString {

    fn new() -> Self {
        return BitString {
            data: vec![0],
            bit_pointer: 0b1000_0000, // 10000000
            elem_pointer: 0,
            bits_set: 0,
            iter_elem_p: 0
        };
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn base10_bit_pointer(&self) -> u8 {
        match self.bit_pointer {
            0b1000_0000 => 0,
            0b0100_0000 => 1,
            0b0010_0000 => 2,
            0b0001_0000 => 3,
            0b0000_1000 => 4,
            0b0000_0100 => 5,
            0b0000_0010 => 6,
            0b0000_0001 => 7,
            _ => panic!("Non-defined state! BitString implementation error!")
        }
    }

    fn bit_len(&self) -> usize {
        // Number of set bits
        let mut p = 0b1000_0000;

        self.data.len() * 8
    }


    fn set_bit(&mut self, b: bool) {
        if b {
            self.data[self.elem_pointer] |= self.bit_pointer;
        }
        if self.bit_pointer == 0b0000_0001 {
            self.bit_pointer = 0b1000_0000;
            self.elem_pointer += 1;
            self.data.push(0);
        }
        else { self.bit_pointer >>= 1; }
    }

    fn unset(&mut self, u: usize) { // Probably not a good idea to just use a usize
        if self.bit_len() < u {
            panic!("Cannot unset a number of bits less than the number of set bits");
        }
        for _ in 0..u {
            if self.bit_pointer == 0b1000_0000 {
                self.elem_pointer -= 1;
                self.bit_pointer = 0b0000_0001;
                self.data.pop();
            }
        }
    } 

    fn append(&mut self, b: BitString) {
        //Consume another BitString by extending this one.
    }

}

impl Iterator for BitString {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_elem_p == self.data.len() {
            self.iter_elem_p = 0;
            None
        } else {
            let d = self.data[self.iter_elem_p];
            self.iter_elem_p += 1;
            Some(d)
        }
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

fn get_coding_scheme(head_node: &Node<char>) -> HashMap<char, BitString> {
    let mut code_map: HashMap<char, BitString> = HashMap::new();
    let bs = BitString::new();
    if head_node.left == None && head_node.right == None {
        bs.set_bit(false);
        code_map.insert(head_node.data.unwrap().clone(), bs.clone());
        return code_map;
    }
    let mut stack: Vec<(&Node<char>, usize, bool)> = vec![(head_node, 0, false)]; 
    let mut p_l: usize = 0;
    while stack.len() != 0 {
        let (node, l, t) = stack.pop().unwrap();
        if p_l > l {
            bs.unset(p_l - l);  // Unset N bits
        }
        bs.set_bit(t);
        if node.left == None && node.right == None {
            code_map.insert(node.data.unwrap().clone(), bs.clone());
        } else {
            if node.left != None {
                stack.push((&node.left.unwrap(), l + 1, false));
            }
            if node.right != None {
                stack.push((&node.right.unwrap(), l + 1, true));
            }
        }
        p_l = l;
    }
    return code_map;
}

fn construct_huffman_tree(f: &HashMap<char, usize>) -> Node<char> {
    let mut heap = BinaryHeap::new();
    f.iter()
        .for_each(|(c, i)| { heap.push(
            Reverse(
                Node {
                    left: None,
                    right: None,
                    data: Some(c.clone()),
                    weight: i.clone()
                }
            )
        ) });
    for _ in 0..heap.len()-2 { // Not sure about this?
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

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::cmp::{Ord, Ordering};

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
        let Reverse(first_node )= heap.pop().unwrap();
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
    println!("Hello, world!");
}

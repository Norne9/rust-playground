use std::cmp::Ordering;

#[derive(Debug)]
enum Node {
    Leaf {
        value: i32,
        left: Box<Node>,
        right: Box<Node>,
    },
    End
}

impl Node {
    fn new(arr: &mut [i32]) -> Node {
        arr.sort();
        Node::build_tree(arr)
    }
    fn build_tree(arr: &[i32]) -> Node {
        match arr.len() {
            0 => Node::End,
            1 => Node::one_value(arr[0]),
            2 => Node::two_values(arr[0], arr[1]),
            len => {
                let center = len / 2;
                Node::Leaf {
                    value: arr[center],
                    left: Box::new(Node::build_tree(&arr[..center])),
                    right: Box::new(Node::build_tree(&arr[center+1..])),
                }
            }
        }
    }
    fn one_value(val: i32) -> Node {
        Node::Leaf {
            value: val,
            left: Box::new(Node::End),
            right: Box::new(Node::End),
        }
    }
    fn two_values(val1: i32, val2: i32) -> Node {
        let (big, small) = if val1 > val2 {
            (val1, val2)
        } else {
            (val2, val1)
        };
        Node::Leaf {
            value: big,
            left: Box::new(Node::one_value(small)),
            right: Box::new(Node::End),
        }
    }
    
    fn contains(&self, v: i32) -> bool {
        match self {
            Node::End => false,
            Node::Leaf { value, left, right } => match v.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.contains(v),
                Ordering::Greater => right.contains(v),
            }
        }
    }
}

fn main() {
    let mut test_data: Vec<_> = (1..20).collect();
    let test2 = Node::new(&mut test_data);
    println!("{:#?}", test2);
    println!("{} {}", test2.contains(18), test2.contains(21));
}

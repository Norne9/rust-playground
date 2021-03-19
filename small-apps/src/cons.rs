#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> where
    T: Clone
{
    fn new(list: &[T]) -> Self {
        if list.len() == 0 {
            List::Nil
        } else {
            List::Cons(list[0].clone(), Box::new(List::new(&list[1..])))
        }
    }
}

struct ListIterator<T> {
    list: Option<List<T>>
}

impl<T> Iterator for ListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.list.take() {
            Some(List::Nil) => None,
            Some(List::Cons(val, list)) => {
                self.list = Some(*list);
                Some(val)
            },
            None => None,
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            list: Some(self)
        }
    }
}

fn main() {
    let mut data: Vec<i32> = (0..20).collect();
    let test = List::new(&mut data);
    for i in test.into_iter() {
        println!("{}", i);
    }

    let mut data: Vec<String> = (0..20)
        .map(|i| format!("i{:02}", i))
        .collect();
    let test = List::new(&mut data);
    for i in test.into_iter() {
        println!("{}", i);
    }
}
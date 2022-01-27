use std::cmp::Ordering;


// while 
fn bSearch<T: Ord>(item: &T, vec: &Vec<T>) -> Option<usize> {
    let mut left = 0;
    let mut right = vec.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match item.cmp(&vec[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1
        }
    }
    None
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    match bSearch(&3, &vec) {
        Some(x) => println!("find is {}", x),
        None => println!("oh gg") 
    }

    match bSearch(&5, &vec) {
        Some(x) => println!("find is {}", x),
        None => println!("oh gg") 
    }
}
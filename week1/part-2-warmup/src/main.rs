/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut vec = Vec::new();
    v.iter().for_each(|x| vec.push(x+n));
    vec
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    v.iter_mut().for_each(|x| *x+=n);
}

fn dedup(v: &mut Vec<i32>) {
    let mut set:HashSet<_> = HashSet::new();
    let mut mark = Vec::new();
    let mut i = 0;
    while i < v.len() {
        if set.insert(v[i]) {
            mark.push(v[i]);
        }
        i += 1;
    }
    v.clear();
    v.extend(mark);
}

// fn dedup(v: &mut Vec<i32>) {
//     let mut vec:Vec<i32> = Vec::new();
//     // for _ in collection中,for循环会直接抢走集合的所有权
//     // 需要使用 for _ in &collection
//     // 这里传进来的是个指针,直接for in v rust会自动解引用;
//     // 但是如果需要声明是不可变引用的话，需要手动解引用,再创建&
//     for ele in & *v {
//         if !vec.contains(ele) {
//             vec.push(*ele);
//         }
//     }
//     v.clear();
//     v.append(&mut vec);
// }


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}

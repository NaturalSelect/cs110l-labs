use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    println!("==========test iterator=========");

    let mut ls = LinkedList::new();
    ls.push_front(10);
    ls.push_front(9);
    ls.push_front(8);
    ls.push_front(7);

    println!("==========test ref iterator=========");

    for ele in &ls {
        println!("{}", ele)
    }

    println!("==========test clone=============");
    let ls_copy: LinkedList<i32> = ls.clone();
    for ele in &ls_copy {
        println!("{}", ele)
    }
    println!("==========test into iterator=========");
    for ele in ls {
        println!("{}", ele)
    }
}

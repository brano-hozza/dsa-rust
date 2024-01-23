use dsa_rust::List;

fn main() {
    let mut list = List::new();
    list.push(1);
    println!("list: {:?}", list);
    println!("list.peek(): {:?}", list.peek());
    let poped = list.pop();
    println!("list.pop(): {:?}", poped);
    println!("list: {:?}", list);
    
}

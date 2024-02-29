use std::collections::{LinkedList, VecDeque};

fn main() {
    println!("This are the data structs!");

    println!("\nSequences\n");
    // Vector - A growable array
    println!("Vector");
    let v: Vec<String> = vec![
        "Gustavo".to_string(),
        "Alfredo".to_string(),
        "Gonzalez".to_string(),
    ];
    println!("{:?}", v);

    // Double end queue - can be used as a stack or a queue
    println!("Dequeue");
    let mut deque: VecDeque<String> = VecDeque::new();
    deque.push_front("Gustavo".to_string());
    deque.push_back("Alfredo".to_string());
    deque.push_front("Gonzalez".to_string());
    println!("{:?}", deque);

    // Linked List - can be used as a stack or a queue
    println!("Linked List");
    let mut list: LinkedList<String> = LinkedList::new();
    list.push_back("Gustavo".to_string());
    list.push_front("Alfredo".to_string());
    list.push_front("Gonzalez".to_string());
    println!("{:?}", list);

    println!("\nMaps\n");
    // HashMap - Has no particular order and is implemented as a hash table
    println!("HashMap");
    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    map.insert("name".to_string(), "Gustavo".to_string());
    map.insert("middle_name".to_string(), "Alfredo".to_string());
    map.insert("last_name".to_string(), "Gonzalez".to_string());
    println!("{:?}", map);

    // BTreeMap - Has a particular order and is implemented as a BTree
    println!("BTreeMap");
    let mut bmap: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    bmap.insert("name".to_string(), "Gustavo".to_string());
    bmap.insert("middle_name".to_string(), "Alfredo".to_string());
    bmap.insert("last_name".to_string(), "Gonzalez".to_string());
    println!("{:?}", bmap);

    println!("\nSets\n");
    // HashSet - Has no particular order and is implemented as a hash table
    println!("HashSet");
    let mut set: std::collections::HashSet<String> = std::collections::HashSet::new();
    set.insert("Gustavo".to_string());
    set.insert("Alfredo".to_string());
    set.insert("Gonzalez".to_string());
    println!("{:?}", set);

    // BTreeSet - Has a particular order and is implemented as a BTree
    println!("BTreeSet");
    let mut bset: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    bset.insert("Gustavo".to_string());
    bset.insert("Alfredo".to_string());
    bset.insert("Gonzalez".to_string());
    println!("{:?}", bset);

    println!("\nOther\n");
    // BinaryHeap - A priority queue
    println!("BinaryHeap");
    let mut heap: std::collections::BinaryHeap<String> = std::collections::BinaryHeap::new();
    heap.push("Gustavo".to_string());
    heap.push("Alfredo".to_string());
    heap.push("Gonzalez".to_string());
    println!("{:?}", heap);
}

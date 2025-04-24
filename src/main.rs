use linked_list_impls::dynamic_linked_list::DynamicLinkedList;
use linked_list_impls::static_linked_list::StaticLinkedList;
use linked_list_impls::LinkedListTrait;

fn main() {
    // Example usage of DynamicLinkedList
    let mut dynamic_list: DynamicLinkedList<i32> = DynamicLinkedList::new();
    dynamic_list.insert(1);
    dynamic_list.insert(2);
    dynamic_list.insert(3);

    println!("Dynamic List: {:?}", dynamic_list);

    // Example usage of StaticLinkedList
    let mut static_list: StaticLinkedList<i32, 5> = StaticLinkedList::new();
    static_list.insert(4);
    static_list.insert(5);
    static_list.insert(6);

    println!("Static List: {:?}", static_list);
}

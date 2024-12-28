fn main() {
    let fruit = vec!["kiwi", "banana", "strawberry"];
    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, "lime");
}

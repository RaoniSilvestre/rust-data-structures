use rusty_data_structures::ds::Queue;

fn main() {
    let mut queue: Queue<i32> = Queue::new();

    queue.enqueue(5);
    queue.enqueue(6);
    queue.enqueue(123);
    queue.enqueue(42);

    queue.dequeue();

    println!("{}", queue);
}

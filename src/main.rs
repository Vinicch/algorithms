mod bubble;
mod quick;

fn main() {
    let mut arr = [3, 2, 5, 78, 56, 14, 32, 8, 5, 6];

    bubble::bubble_sort(&mut arr);

    println!("Bubble: {arr:?}");

    quick::quick_sort(&mut arr);

    println!("Quick: {arr:?}");
}

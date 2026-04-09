// fun with vectors

// compile and run:
// rustc -o vec.out vec.rs && ./vec.out && rm -f vec.out

fn f(v: Vec<i32>) {
    v
        .into_iter()
        .for_each(|el| {
            println!("{}", el);
        });
}

fn main() {
    f(vec![1, 2, 3, 4, 5]);
}

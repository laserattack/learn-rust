// fun with heap values

// compile and run:
// rustc -o heap.out heap.rs && ./heap.out && rm -f heap.out

fn main() {
    let mut val = Box::<u32>::new_uninit(); // box - значение в куче
    val.write(5);
    val.write(10);
    val.write(1337);
}

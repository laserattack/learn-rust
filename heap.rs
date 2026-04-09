// fun with heap values

// compile and run:
// rustc -o heap.out heap.rs && ./heap.out && rm -f heap.out

fn main() {
    let big_box = Box::<[usize; 1024]>::new_uninit();

    let mut array = [0; 1024];
    for (i, place) in array.iter_mut().enumerate() {
        *place = i;
    }

    // file:///home/serr/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/boxed/struct.Box.html#method.write
    // Оптимизатор может исключить это копирование, так что предыдущий код (цикл) будет писать прямо в кучу.
    let big_box = Box::write(big_box, array);

    for (i, x) in big_box.iter().enumerate() {
        assert_eq!(*x, i);
    }

    // гарантированно писать в сразу кучу можно так (приходится использовать unsafe)
    let mut big_box = Box::<[usize; 1024]>::new_uninit();
    let ptr = big_box.as_mut_ptr() as *mut usize;
    for i in 0..1024 {
        unsafe { ptr.add(i).write(i); }
    }
    let big_box = unsafe { big_box.assume_init() };
    for (i, x) in big_box.iter().enumerate() {
        assert_eq!(*x, i);
    }

    // ну или просто использовать вектор!
}

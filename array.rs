// fun with arrays

// compile and run:
// rustc -o array.out array.rs && ./array.out && rm -f array.out

fn main() {
    // трейт copy реализуется, но определен так
    // impl<T, const N: usize> Copy for [T; N]
    // where
    //     T: Copy,
    // т.е. трейт copy реализуется только если элементы реализуют трейт copy

    // вот например i32 реализет трейт copy
    let a = [1, 2, 3];
    let b = a;
    println!("{:?} {:?}", a, b); // [1, 2, 3] [1, 2, 3]

    // а вот тут уже
    // let a = [String::from("hello"), String::from("world")];
    // let b = a;
    // println!("{:?}", a);

    // будет такое

    //       --> array.rs:20:22
    //    |
    // 18 |     let a = [String::from("hello"), String::from("world")];
    //    |         - move occurs because `a` has type `[String; 2]`, which does not implement the `Copy` trait
    // 19 |     let b = a;
    //    |             - value moved here
    // 20 |     println!("{:?}", a);
    //    |                      ^ value borrowed here after move

    // потому что String не реализует трейт Copy
}

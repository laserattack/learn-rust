// fun with refs

// compile and run:
// rustc -o refs.out refs.rs && ./refs.out && rm -f refs.out

fn f_str(s: &str) {
    println!("{}", s);
}

// fn f_vec(v: Vec<i32>) {
//     v
//         .into_iter()
//         .for_each(|el| {
//             println!("{}", el);
//         });
// }

fn main() {
    // нельзя!
    // let v = vec![1,2,3];
    // f_vec(v);
    // println!("{:?}", v);

    // можно! ссылка не владеет данными (реализует трейт Copy)
    // doc/rust/html/std/primitive.reference.html#trait-implementations-1
    let p: &str = "123";
    f_str(p);
    println!("{}", p);
}

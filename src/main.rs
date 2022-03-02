// const MI_CONST:i32 = 1;

fn main() {
    // let mut x  = 99;
    // println!("x is: {}", x);
    // x = 2;
    // println!("x is: {}", x);

    // println!("MI_CONST is: {}", MI_CONST);
    // let (x, y, z) = (1, 2, 3);
    // println!("x is: {}", x);
    // println!("y is: {}", y);
    // println!("z is: {}", z);

    // let ejemplo = Ejemplo{
    //     a: 1,
    //     b: 2,
    // };
    // println!("ejemplo.a is: {}", ejemplo.a);
    // println!("ejemplo.b is: {}", ejemplo.b);
    // println!("ejemplo is: {:?}", ejemplo);

    let Ejemplo { a: x, b: y } = Ejemplo { a: 1, b: 2 };
    println!("a is: {}", x);
    println!("b is: {}", y);
}

// fn f()-> i32{
//     43
// }

#[derive(Debug)]
struct Ejemplo{
    a: i32,
    b: i32,
}
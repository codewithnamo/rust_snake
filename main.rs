

fn main() {
    let float_num: f32 = 3.14;
    let float_num_2: f64 =3.2334327489;

    let tuple: (i32, &str, u8) = (20, "Hello", 1); 

    println!("{}", tuple.1); 

    let (a,b,c) = tuple; 
    println!("{}", a);

    let x = [1, 5, 6, 7];

    println!("{}", x[2]);

    let y = [2; 6]; // [2, 2, 2, 2, 2, 2]
    println!("{}", y[5]);
}






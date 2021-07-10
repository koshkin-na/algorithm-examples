

use rand::Rng;





fn main() {
    println!("Hello, world!");
    let mut vec_number: Vec<i32> = Vec::new();
    let mut rnd = rand::thread_rng();
    for i in 0..10 { vec_number.push(rnd.gen_range(0..100)) }


    println!("{:?}", vec_number);
}

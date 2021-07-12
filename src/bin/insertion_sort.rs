use rand::Rng;


fn main() {
     let mut vec_number: Vec<i32> = Vec::new();
    let mut rnd = rand::thread_rng();
     for _ in 0..100 { vec_number.push(rnd.gen_range(0..100)); }
    println!("Raw random vector (number): \n{:?}", vec_number);
    insertion_sort(&mut vec_number);
    println!("After sorting: \n{:?}", vec_number);
}


fn insertion_sort<T: Ord + Copy>(val: &mut Vec<T>) {
    // let mut count = 0;
    for i in 0..val.len() {
        let x = val[i];
        let mut j = i;
        while j > 0 && x < val[j-1] {
            // count += 1;
            val.swap(j, j-1);
            j -= 1;
        }
    }
    // println!("Algorithm operation count: {}", count);
}

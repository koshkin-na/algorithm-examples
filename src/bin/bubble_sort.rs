use rand::Rng;


fn main() {
    println!("An example of a bubble sort implementation ");
    let mut vec_number: Vec<i32> = Vec::new();
    let mut rnd = rand::thread_rng();
    for _ in 0..50 { vec_number.push(rnd.gen_range(0..100)) }
    println!("Raw random vector (number): \n{:?}", vec_number);
    bubble_sort(&mut vec_number);
    println!("After sorting: \n{:?}", vec_number);

    let mut vec_string: Vec<String> = vec![
        "Maxim".to_string(),
        "Nikolay".to_string(),
        "Alex".to_string(),
        "Sergey".to_string(),
        "Volodya".to_string(),
        "Alex".to_string(),
        "Sergey".to_string(),
    ];
    println!("Raw random vector (string): \n{:?}", vec_string);
    bubble_sort(&mut vec_string);
    println!("After sorting: \n{:?}", vec_string);
}


fn bubble_sort<T: Ord>(vec: &mut Vec<T>) {
    // let mut count = 0;
    for i in 0..vec.len() {
        for j in i..vec.len() {
            // count += 1;
            if vec[i] > vec[j] {
                vec.swap(i, j);
            }
        }
    }
    // println!("Algorithm operation count: {}", count);
}
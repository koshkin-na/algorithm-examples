use rand::Rng;
use simplelog::{Config, SimpleLogger};
use std::time::Duration;
use cpu_time::ProcessTime;

const SIZE_ARRAY: usize = 10_000_000;

fn main() {
    let _ = SimpleLogger::init(log::LevelFilter::Info, Config::default());
    log::info!("An example of a heap sort implementation");
    let mut arr: Vec<i32> = Vec::with_capacity(SIZE_ARRAY);
    let mut rnd = rand::thread_rng();
    for _ in 0..SIZE_ARRAY { arr.push(rnd.gen_range(1..100)); }
    let mut arr_clone = arr.clone();
    if arr.len() < 20 { log::info!("{:?}", arr); }
    let start = ProcessTime::now();
    heap_sort(&mut arr);
    let cpu_time: Duration = start.elapsed();
    log::info!("heap_sort() CPU time elapsed is {:?}", cpu_time);
    if arr.len() < 20 { log::info!("{:?}", arr); }

    let start = ProcessTime::now();
    arr_clone.sort();
    let cpu_time: Duration = start.elapsed();
    log::info!("native sort CPU time elapsed is {:?}", cpu_time);
}

fn heap_sort(arr: &mut [i32]) {
    for i in (0..=(arr.len() / 2)).rev() {
        max_heapify(arr, i, arr.len() - 1);
    }
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        max_heapify(arr, 0, i - 1);
    }
}

fn max_heapify(arr: &mut [i32], start: usize, end: usize) {
    let mut child = start * 2 + 1;
    if child > end { return }
    if child < end && arr[child] < arr[child + 1] {
        child += 1;
    }
    if arr[start] < arr[child] {
        arr.swap(start, child);
        max_heapify(arr, child, end);
    }

    // Kormen implementation
    // let (l, r) = (i * 2 + 1, i * 2 + 2);
    // let largest = if  l < j && arr[l] > arr[i] { l } else { i };
    // let largest = if r < j && arr[r] > arr[largest] { r } else { largest };
    // if largest != i {
    //     arr.swap(i, largest);
    //     max_heapify(arr, largest, j);
    // }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        use rand::Rng;
        let mut vec_number: Vec<i32> = Vec::new();
        let mut rnd = rand::thread_rng();
        for _ in 0..10000 {
            vec_number.push(rnd.gen_range(0..100))
        }
        let mut vec_number_clone = vec_number.clone();
        heap_sort(&mut vec_number);
        vec_number_clone.sort();
        assert_eq!(vec_number, vec_number_clone);
    }
}
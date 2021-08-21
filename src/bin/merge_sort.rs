use rand::Rng;
use std::fmt::Debug;
use std::time::Duration;
use cpu_time::ProcessTime;
use simplelog::{Config, SimpleLogger};

const RANGE_BEHCMARK: i64 = 100_000_000;
// const RANGE_BEHCMARK: i64 = 10;

fn main() {
    let _ = SimpleLogger::init(log::LevelFilter::Info, Config::default());
    log::info!("An example of a merge sort implementation");
    
    let mut vec_number: Vec<i32> = Vec::new();
    let mut rnd = rand::thread_rng();
    for _ in 0..RANGE_BEHCMARK { vec_number.push(rnd.gen_range(0..1000)) }
    let mut vec_number_clone = vec_number.clone();
    if vec_number.len() < 20 { log::info!("{:?}", vec_number); }

    let start = ProcessTime::now();
    merge_sort(&mut vec_number);
    let cpu_time: Duration = start.elapsed();
    log::info!("merge_sort() CPU time elapsed is {:?}", cpu_time);
    if vec_number.len() < 20 { log::info!("{:?}", vec_number); }

    let start = ProcessTime::now();
    vec_number_clone.sort();
    let cpu_time: Duration = start.elapsed();
    log::info!("native Vector::sort() CPU time elapsed is  {:?}", cpu_time);
}

fn merge_sort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    log::debug!("call merge_sort({:?})", arr);
    let arr_len = arr.len();
    let mid = arr_len / 2;
    if arr_len <= 1 { return }
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..arr_len]);
    let mut res: Vec<T> = arr.to_vec();
    merge(
        &arr[0..mid], 
        &arr[mid..arr_len], 
        &mut res[..]
    );
    arr.copy_from_slice(&res);
}

fn merge<T: Copy + PartialOrd + Debug>(x1: &[T], x2: &[T], y: &mut [T]) {
    log::debug!("call merge({:?}, {:?}, {:?})", x1, x2, y);
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use rand::Rng;
        let mut vec_number: Vec<i32> = Vec::new();
        let mut rnd = rand::thread_rng();
        for _ in 0..10000 {
            vec_number.push(rnd.gen_range(0..100))
        }
        let mut vec_number_clone = vec_number.clone();
        crate::merge_sort(&mut vec_number);
        vec_number_clone.sort();
        assert_eq!(vec_number, vec_number_clone);
    }
}
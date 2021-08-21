use rand::Rng;
use std::fmt::Debug;
use std::time::Duration;
use cpu_time::ProcessTime;
use simplelog::{Config, SimpleLogger};


const RANGE_BEHCMARK: i64 = 100_000_000;
// const RANGE_BEHCMARK: i64 = 100;


fn main() {
    let _ = SimpleLogger::init(log::LevelFilter::Info, Config::default());
    log::info!("An example of a merge sort implementation");
    
    let mut vec_number: Vec<i32> = Vec::new();
    let mut rnd = rand::thread_rng();
    for _ in 0..RANGE_BEHCMARK {
        vec_number.push(rnd.gen_range(0..100))
    }
    if vec_number.len() < 20 {
        log::info!("{:?}", vec_number);
    }

    let start = ProcessTime::now();
    let res = merge_sort(&vec_number);
    let cpu_time: Duration = start.elapsed();
    log::info!("merge_sort() CPU time elapsed is {:?}", cpu_time);
    if vec_number.len() < 20 {
        log::info!("{:?}", res);
    }

    let start = ProcessTime::now();
    vec_number.sort();
    let cpu_time: Duration = start.elapsed();
    log::info!("native Vector::sort() CPU time elapsed is  {:?}", cpu_time);
}

fn merge_sort<T: Ord + Sized + Debug + Clone + Copy>(array: &[T]) -> Vec<T> {
    log::debug!("call merge_sort({:?})", array);
    if array.len() > 1 {
        let part_left = merge_sort(&array[0 .. (array.len() / 2)]);
        let part_right = merge_sort(&array[(array.len() / 2) .. ]);
        return merge(&part_left, &part_right)

    } else {
        return array.to_vec()
    }
}

fn merge<T: Ord + Sized + Debug + Clone + Copy>(part_left: &[T], part_right: &[T]) -> Vec<T> {
    log::debug!("call merge({:?}, {:?})", part_left, part_right);
    let mut result = Vec::new();
    let mut left_i: usize = 0;
    let mut right_i: usize = 0;
    loop {
        if part_left.len() - 1 < left_i || part_right.len() - 1 < right_i { break }
        if part_left[left_i] < part_right[right_i] {
            result.push(part_left[left_i]);
            left_i += 1;
        } else {
            result.push(part_right[right_i]);
            right_i += 1;
        }
    }
    if part_left.len() > left_i {
        result.append(&mut part_left[left_i..].to_vec())
    }
    if part_right.len() > right_i {
        result.append(&mut part_right[right_i..].to_vec())
    }
    return result
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use rand::Rng;
        let mut vec_number: Vec<i32> = Vec::new();
        let mut rnd = rand::thread_rng();
        for _ in 0..100 {
            vec_number.push(rnd.gen_range(0..100))
        }
        let res = crate::merge_sort(&vec_number);
        vec_number.sort();
        assert_eq!(res, vec_number);
    }
}
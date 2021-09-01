use rand::Rng;
use simplelog::{Config, SimpleLogger};
use std::time::Duration;
use cpu_time::ProcessTime;

const SIZE_ARRAY: usize = 15;

fn main() {
    let _ = SimpleLogger::init(log::LevelFilter::Info, Config::default());
    log::info!("An example of a find max subarray implementation");
    let mut arr: Vec<i32> = Vec::with_capacity(SIZE_ARRAY);
    let mut rnd = rand::thread_rng();
    for _ in 0..SIZE_ARRAY { arr.push(rnd.gen_range(-1000..1000)); }
    if arr.len() < 20 { log::info!("{:?}", arr); }
    let start = ProcessTime::now();
    let res = find_max_subarr_linear(&arr);
    let cpu_time: Duration = start.elapsed();
    log::info!("merge_sort() CPU time elapsed is {:?}", cpu_time);
    if res.len() < 20 { log::info!("{:?}", res); }
}

/// Algorithm complexity: O(n)
fn find_max_subarr_linear(arr: &[i32]) -> &[i32] {
    let mut slice: (usize, usize) = (0, 0);
    let mut start_index: usize = 0;
    let mut max_sum: i32 = i32::MIN;
    let mut curr_sum: i32 = 0;
    for (i, v) in arr.iter().enumerate() {
        curr_sum = curr_sum + v;
        if curr_sum < 1 {
            start_index = i;
            curr_sum = 0;
            continue;
        }
        if curr_sum > max_sum {
            max_sum = curr_sum;
            slice.1 = i;
            slice.0 = start_index + 1;
        }
    }
    log::debug!("start_index: {}, end_index: {}", slice.0, slice.1);
    return &arr[slice.0..=slice.1]
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let arr = vec![23, -78, 76, 0, 58, 48, -30, -70, -86, 94];
        let res = find_max_subarr_linear(&arr);
        assert_eq!(res, &[76, 0, 58, 48]);
    }
}
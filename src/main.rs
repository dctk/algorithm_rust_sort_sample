pub mod sort;

extern crate rand;
use rand::Rng;
use rand::prelude::*;
use std::time::{Instant};

macro_rules! proc_time {
	($proc:expr) => {
		let begin = Instant::now();
		let result = $proc;
		let end = begin.elapsed();

		println!("処理時間：{:?}",end);
	};
}

#[allow(dead_code)]
fn main() {
    let mut rng = rand::thread_rng();

    // バブルソート
    {
        println!("バブルソート");
        let mut data:Vec<u32> = Vec::new();
        for i in 0..10000 {
            data.push(rng.next_u32());
        }

        proc_time!({
            sort::bubble_sort(&mut data);
        });
    } // 5.033569613s

    // クイックソート
    {
        println!("クイックソート");
        let mut data:Vec<u32> = Vec::new();
        for i in 0..10000 {
            data.push(rng.next_u32());
        }

        proc_time!({
            let len =data.len();
            sort::quick_sort(&mut data,0,len-1);
        });
    } // 10.754091ms

    // マージソート
    {
        println!("マージソート");
        let mut data:Vec<u32> = Vec::new();
        for i in 0..10000 {
            data.push(rng.next_u32());
        }

        proc_time!({
            let len =data.len();
            sort::merge_sort(&mut data,0,len-1);
        });
    } // 49.834005ms

	// コームソート
	{
		println!("コームソート");
		let mut data:Vec<u32> = Vec::new();
        for i in 0..10000 {
            data.push(rng.next_u32());
        }

        proc_time!({
            sort::comb_sort(&mut data);
        });
	} // 29.062876ms

	// インサートソート
	{
		println!("インサートソート");
		let mut data:Vec<u32> = Vec::new();
        for i in 0..10000 {
            data.push(rng.next_u32());
        }

        proc_time!({
            sort::insert_sort(&mut data);
        });
	} // 1.939848485s
}

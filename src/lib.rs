mod bucket;
use bucket::{Bucket};

const MAXKICKS: i32 = 500;


pub struct CuckooFilter {
    buckets: Vec<Bucket>,
    hash: Box<dyn Fn(Vec<u8>) + 'static + Send + Sync>,
    nBuckets: usize,
    nEntry: usize,
    fpLength: usize,
    count: usize,
    capacity: usize,
}
impl CuckooFilter {
    pub fn new(capacity: usize, fpRate: f64) -> CuckooFilter {
        let nEntry = 4;
        let fpLength =

        let fpLength = 4;
        let nBuckets = capacity.next_power_of_two();

        let buckets: Vec<Bucket> = Vec: new();

        for _ in 0..100 {
            buckets.push(Bucket{});
        }

        CuckooFilter{
            buckets: buckets,
            hash


        }
    }
}

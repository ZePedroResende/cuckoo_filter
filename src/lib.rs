mod bucket;
use bucket::Bucket;

const MAXKICKS: i32 = 500;

pub struct CuckooFilter {
    buckets: Vec<Bucket>,
    //    hash: Box<dyn Fn(Vec<u8>) + 'static + Send + Sync>,
    bucket_size: usize,
    num_buckets: usize,
    filled: usize,
    capacity: usize,
}

impl CuckooFilter {
    pub fn new(capacity: usize, fpRate: f64) -> CuckooFilter {
        let mut buckets: Vec<Bucket> = Vec::new();

        for _ in 0..100 {
            buckets.push(Bucket::new())
        }

        CuckooFilter {
            buckets: buckets,
            //   hash:
            bucket_size: 8,
            num_buckets: 4 << 20,
            filled: 0,
            capacity: capacity,
        }
    }

    pub fn insert<M: Into<Vec<u8>>>(&self, m: M) -> bool {
        let value = m.into();
        let fingerprint : u8 = fingerprint(value);

    }
    pub fn contains<M: Into<Vec<u8>>>(&self, m: M) -> bool {
        let value = m.into();
        unimplemented!()
    }
    pub fn remove<M: Into<Vec<u8>>>(&self, m: M) -> bool {
        let value = m.into();
        unimplemented!()
    }

    pub fn fingerprint(value: Vec<u8>) -> u8 {

        unimplemented!()
    }
}

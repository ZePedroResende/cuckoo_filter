const MAXRELOC: i32 = 500;

type BucketVector = Vec<Option<Vec<u8>>>;

pub struct Bucket {
    bucket: BucketVector,
}

impl Bucket {
    pub fn contains(&self, value: Option<Vec<u8>>) -> bool {
        let mut iter = self.bucket.iter();
        iter.position(|v| v == &value) != None
    }

    pub fn index_of(&self, value: Option<Vec<u8>>) -> Option<usize> {
        let mut iter = self.bucket.iter();
        iter.position(|v| v == &value)
    }

    pub fn get_empty_entry(&self) -> Result<usize, &'static str> {
        match self.index_of(None) {
            None => Err("full"),
            Some(v) => Ok(v),
        }
    }
}

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
        let fpLenth =

        let fpLength = 4;
        let nBuckets = capacity.next_power_of_two();

        CuckooFilter {
            buckets: Vec:new,
        }


        let buckets  : Vec<Bucket> = Vec:new();

        for _ in 0..100 {

        }
    }
}

mod bucket;
use bucket::Bucket;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::seq::SliceRandom;
use std::rand::{task_rng, Rng};

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
        let first_index = ;
        let second_index = ;

        if self.buckets[first_index].insert(fingerprint) || self.buckets[second_index].insert(fingerprint) {
            self.filled++;
            True
        }

        let v = vec![first_index, second_index];
        let rand_index = v.choose(&mut rand::thread_rng());

        for try in 0..MAXKICKS {
            let num: usize = task_rng().gen_range(0,bucket_size);
            removed_fp = fingerprint;
            fingerprint = self.buckets[rand_index].bucket[num];
            self.buckets[rand_index].bucket[num] = removed_fp;

             rand_index = ;

            if self.buckets[rand_index].insert(fingerprint) {
                self.filled++;
                True
            }
        }

        False
        
    }
    pub fn contains<M: Into<Vec<u8>>>(&self, m: M) -> bool {
        let value = m.into();
        let fingerprint : u8 = fingerprint(value);
        let first_index = ;
        let second_index = ;

        self.buckets[first_index].contains(fingerprint) || self.buckets[second_index].contains(fingerprint)

    }

    pub fn remove<M: Into<Vec<u8>>>(&self, m: M) -> bool {
        let value = m.into();
        let fingerprint : u8 = fingerprint(value);
        let first_index = ;
        let second_index = ;

        if self.buckets[first_index].delete(fingerprint) || self.buckets[second_index].delete(fingerprint) {
            self.filled++;
            True
        }

        False
    }

    fn fingerprint(value: Vec<u8>) -> [u8; 8] {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish() ;
        let hasher = hash % 255 + 1;
        hasher.to_ne_bytes()
    }

    fn alt_index(fingerprint: u8, index: usize, num_bucket: usize) -> usize{
        let mut hasher = DefaultHasher::new();
        fingerprint.hash(&mut hasher);
        let hash = hasher.finish() ;
        (index ^ hash as usize) % num_bucket
    }
}

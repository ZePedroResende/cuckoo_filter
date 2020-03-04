mod bucket;
use bucket::Bucket;
use rand::{thread_rng, Rng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const MAXKICKS: i32 = 500;

pub struct CuckooFilter {
    buckets: Vec<Bucket>,
    bucket_size: usize,
    num_buckets: usize,
    filled: usize,
}

impl CuckooFilter {
    pub fn new(capacity: usize) -> CuckooFilter {
        let mut buckets: Vec<Bucket> = Vec::new();

        for _ in 0..100 {
            buckets.push(Bucket::new(capacity))
        }

        CuckooFilter {
            buckets: buckets,
            bucket_size: 8,
            num_buckets: 4 << 20,
            filled: 0,
        }
    }

    pub fn insert<M: Into<Vec<u8>>>(&mut self, m: M) -> bool {
        let value = m.into();
        let (mut fingerprint, index, alt_index) = fingerprint_i1_i2(value, self.num_buckets);

        if self.buckets[index].insert(fingerprint) || self.buckets[alt_index].insert(fingerprint) {
            self.filled += 1;
            return true;
        }

        let mut rand_index = thread_rng().gen_range(index, alt_index);

        for _ in 0..MAXKICKS {
            let num: usize = thread_rng().gen_range(0, self.bucket_size);
            let removed_fp = fingerprint;
            let bucket = self.buckets[rand_index].clone();

            fingerprint = match bucket.bucket[num] {
                Some(x) => x,
                None => return false,
            };

            self.buckets[rand_index].bucket[num] = Some(removed_fp);

            rand_index = thread_rng().gen_range(index, alt_index);

            if self.buckets[rand_index].insert(fingerprint) {
                self.filled += 1;
                return true;
            }
        }

        false
    }

    pub fn contains<M: Into<Vec<u8>>>(&self, m: M) -> bool {
        let value = m.into();
        let (fingerprint, index, alt_index) = fingerprint_i1_i2(value, self.num_buckets);

        let bucket = self.buckets[index].clone();
        let bucket_alt = self.buckets[alt_index].clone();

        bucket.contains(Some(fingerprint)) || bucket_alt.contains(Some(fingerprint))
    }

    pub fn remove<M: Into<Vec<u8>>>(&mut self, m: M) -> bool {
        let value = m.into();
        let (fingerprint, index, alt_index) = fingerprint_i1_i2(value, self.num_buckets);

        if self.buckets[index].delete(fingerprint) || self.buckets[alt_index].delete(fingerprint) {
            self.filled += 1;
            return true;
        }

        false
    }
}

fn alt_index(fingerprint: u8, index: usize, num_bucket: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    fingerprint.hash(&mut hasher);
    let hash = hasher.finish() as usize % num_bucket;
    (index % num_bucket) ^ hash
}

fn fingerprint_i1_i2(value: Vec<u8>, num_bucket: usize) -> (u8, usize, usize) {
    let mut hasher = DefaultHasher::new();

    value.hash(&mut hasher);

    let hash = hasher.finish();
    let hasher = hash % 255 + 1;
    let fingerprint = hasher as u8;

    let index1 = hash as usize % num_bucket;
    let alt_index = alt_index(fingerprint, index1, num_bucket);
    (fingerprint, index1, alt_index)
}

const BUCKET_SIZE: usize = 4;

type BucketVector = Vec<Option<u8>>;

#[derive(Debug)]
pub struct Bucket {
    bucket: BucketVector,
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            bucket: vec![None; BUCKET_SIZE],
        }
    }

    pub fn insert(&mut self, value: u8) -> bool {
        match self.get_empty_entry() {
            Ok(index) => {
                self.bucket[index] = Some(value);
                true
            }
            Err(err) => false,
        }
    }

    pub fn delete(&mut self, value: u8) -> bool {
        match self.index_of(Some(value)) {
            Some(index) => {
                self.bucket[index] = None;
                true
            }
            None => false,
        }
    }

    pub fn contains(&self, value: Option<u8>) -> bool {
        let mut iter = self.bucket.iter();
        iter.position(|v| v == &value) != None
    }

    pub fn index_of(&self, value: Option<u8>) -> Option<usize> {
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

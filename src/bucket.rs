type BucketVector = Vec<Option<u8>>;

#[derive(Debug, Clone)]
pub struct Bucket {
    pub bucket: BucketVector,
}

impl Bucket {
    pub fn new(capacity: usize) -> Bucket {
        Bucket {
            bucket: vec![None; capacity],
        }
    }

    pub fn insert(&mut self, value: u8) -> bool {
        match self.get_empty_entry() {
            Ok(index) => {
                self.bucket[index] = Some(value);
                true
            }
            Err(_) => false,
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

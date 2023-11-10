const TABLE_SIZE: usize = 256; // A constant for the size of the table

// A struct representing an entry in our hash table
#[derive(Clone)]
struct Entry {
    key: String,
    value: String,
}

struct HashTable {
    table: Vec<Option<Vec<Entry>>>, // A vector of options containing vectors of entries
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            table: vec![None; TABLE_SIZE],
        }
    }

    fn hash(&self, key: &str) -> usize {
        key.bytes()
            .fold(0, |acc, b| (acc + b as usize) % TABLE_SIZE)
    }

    // Insert a key-value pair into the hash table
    fn insert(&mut self, key: String, value: String) {
        let idx = self.hash(&key);
        match &mut self.table[idx] {
            Some(bucket) => {
                for entry in bucket.iter_mut() {
                    if entry.key == key {
                        entry.value = value; // Update the value if key already exists
                        return;
                    }
                }
                bucket.push(Entry { key, value }); // Add a new entry if key not found
            }
            None => {
                let bucket = vec![Entry { key, value }];
                self.table[idx] = Some(bucket);
            }
        }
    }

    // Get a value for a given key
    fn get(&self, key: &str) -> Option<&String> {
        let idx = self.hash(key);
        match &self.table[idx] {
            Some(bucket) => {
                for entry in bucket.iter() {
                    if entry.key == key {
                        return Some(&entry.value);
                    }
                }
                None
            }
            None => None,
        }
    }
}

// Test
fn main() {
    let mut ht = HashTable::new();
    ht.insert("identifier".to_string(), "value".to_string());
    ht.insert("constant".to_string(), "42".to_string());

    println!("{:?}", ht.get("identifier"));
    println!("{:?}", ht.get("constant"));
}

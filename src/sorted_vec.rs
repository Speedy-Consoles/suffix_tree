use std::slice;
use std::cmp::Eq;

#[derive (Debug)]
pub struct SortedVec<K, V>
    where K: Eq + Ord,
          V: Default,
{
    data: Vec<(K, V)>,
}

impl<K, V> SortedVec<K, V>
    where K: Eq + Ord,
          V: Default,
{
    pub fn new() -> Self {
        SortedVec { data: Vec::new() }
    }

    pub fn iter(&self) -> slice::Iter<(K, V)> {
        self.data.iter()
    }

    pub fn entry_at(&mut self, index: usize) -> &mut (K, V) {
        &mut self.data[index]
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for e in self.data.iter() {
            if e.0 == *key {
                return Some(&e.1);
            } else if e.0 > *key {
                return None;
            }
        }
        return None;
    }

    /*pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        for e in self.data.iter_mut() {
            if e.0 == key {
                return Some(e);
            } else if e.0 > key {
                return None;
            }
        }
    }*/

    pub fn get_or_default_mut(&mut self, key: K) -> (usize, &mut V) {
        let mut index = 0;
        let mut insert = true;
        for e in self.data.iter_mut() {
            if e.0 == key {
                insert = false;
                break;
            } else if e.0 > key {
                break;
            }
            index += 1;
        }

        if insert {
            self.data.insert(index, (key, Default::default()));
        }
        (index, &mut self.data[index].1)
    }
}
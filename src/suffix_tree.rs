use std::vec::Vec;
use std::collections::HashMap;

use find::Find;

#[derive(Debug)]
struct Thread<'a> {
    start_index: usize,
    text: &'a str,
    // Byte index in the haystack where we have to start to have the needle ending here.
    // This does not include the start point that caused the creation of this node's thread.
    extra_start_points: HashMap<usize, Vec<usize>>,
    children: HashMap<usize, Vec<Thread<'a>>>,
}

#[derive(Debug)]
pub struct SuffixTree<'a> {
    haystack: &'a str,
    start_threads: Vec<Thread<'a>>,
}

impl<'a> Find<'a> for SuffixTree<'a> {
    fn new(haystack: &'a str) -> Self {
        let mut st = SuffixTree {
            haystack,
            start_threads: Vec::new(),
        };

        for start_index in haystack.char_indices().map(|e| e.0) {
            let mut rem_suffix = &haystack[start_index..];
            let mut children = &mut st.start_threads;
            let mut inserted = false;
            while !inserted {
                let mut branch_data = None;
                for (child_index, thread) in children.iter_mut().enumerate() {
                    let thread_offset = diff_pos(thread.text, rem_suffix);
                    if thread_offset == 0 {
                        continue;
                    }
                    for i in 1..(thread_offset + 1) {
                        thread.extra_start_points
                            .entry(i).or_insert(Default::default())
                            .push(start_index);
                    }
                    if thread_offset == rem_suffix.len() {
                        // During creation there will never be a needle
                        // that is longer than the thread. So the needle ends in this thread
                        // and there is no need to create a new thread.
                        inserted = true;
                        break;
                    } else {
                        // The needle goes into another thread
                        rem_suffix = &rem_suffix[thread_offset..];
                        branch_data = Some ((child_index, thread_offset));
                        break;
                    }
                }
                if let Some((child_index, thread_offset)) = branch_data {
                    children = {children}[child_index].children
                        .entry(thread_offset).or_insert(Default::default());
                } else if !inserted {
                    // Creating a new thread for this needle.
                    children.push(Thread {
                        start_index,
                        text: rem_suffix,
                        extra_start_points: HashMap::new(),
                        children: HashMap::new(),
                    });
                    inserted = true;
                }
            }
        }

        return st;
    }

    fn find(&self, needle: &str, start_points: &mut Vec<usize>) {
        start_points.clear();
        let mut rem_needle = needle;
        let mut children = &self.start_threads;
        loop {
            let mut branched = false;
            for thread in children.iter() {
                let thread_offset = diff_pos(thread.text, rem_needle);
                if thread_offset == 0 {
                    continue;
                }
                if thread_offset == rem_needle.len() {
                    start_points.push(thread.start_index);
                    if let Some(extra_start_points) = thread.extra_start_points.get(&thread_offset) {
                        start_points.extend(extra_start_points);
                    }
                    return;
                } else if let Some(new_children) = thread.children.get(&thread_offset) {
                    children = new_children;
                    rem_needle = &rem_needle[thread_offset..];
                    branched = true;
                    break;
                } else {
                    return;
                }
            }
            if !branched {
                break;
            }
        }
    }
}

fn diff_pos(a: &str, b: &str) -> usize {
    a.bytes().zip(b.bytes()).take_while(|(a, b)| a == b).count()
}
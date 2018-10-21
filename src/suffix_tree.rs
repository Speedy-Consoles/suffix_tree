use std::vec::Vec;
use std::collections::HashMap;

use find::Find;

#[derive(Default, Debug)]
struct Node<'a> {
    // Byte index in the haystack where we have to start to have the needle ending here.
    // This does not include the start point that caused the creation of this node's thread.
    extra_start_points: Vec<usize>,
    children: Vec<Thread<'a>>,
}

#[derive(Debug)]
struct Thread<'a> {
    start_index: usize,
    text: &'a str,
    nodes: HashMap<usize, Node<'a>>,
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
                        thread.nodes.entry(i).or_insert(Default::default())
                            .extra_start_points.push(start_index);
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
                    children = &mut {children}[child_index].nodes.get_mut(&thread_offset).unwrap().children;
                } else if !inserted {
                    // Creating a new thread for this needle.
                    children.push(Thread { start_index, text: rem_suffix, nodes: HashMap::new() });
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
                    if let Some(node) = thread.nodes.get(&thread_offset) {
                        start_points.extend(&node.extra_start_points);
                    }
                    return;
                } else if let Some(node) = thread.nodes.get(&thread_offset) {
                    children = &node.children;
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
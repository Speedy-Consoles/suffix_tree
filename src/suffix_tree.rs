use std::vec::Vec;
use std::collections::HashMap;

use find::Find;
use sorted_vec::SortedVec;

#[derive(Debug)]
struct Thread {
    suffix_start: usize,
    thread_start: usize,
    nodes: SortedVec<usize, Node>,
}

#[derive(Debug, Default)]
struct Node {
    branches: HashMap<char, Thread>,
    contained_suffix_start: Option<usize>,
}

#[derive(Debug)]
pub struct SuffixTree<'a> {
    haystack: &'a str,
    start_threads: HashMap<char, Thread>,
}

impl<'a> Find<'a> for SuffixTree<'a> {
    fn new(haystack: &'a str) -> Self {
        let mut st = SuffixTree {
            haystack,
            start_threads: HashMap::new(),
        };

        let mut prev_progress = 0;
        for suffix_start in haystack.char_indices().map(|e| e.0) {
            let ratio = suffix_start as f64 / haystack.len() as f64;
            let progress = (ratio * 100.0) as i64;
            if progress > prev_progress {
                //println!("{}%", progress);
                prev_progress = progress;
            }

            let mut thread_start = suffix_start;
            let mut threads = &mut st.start_threads;
            loop {
                let rem_suffix = &haystack[thread_start..];
                let first_char = rem_suffix.chars().next().unwrap();
                let node_index = if threads.contains_key(&first_char) {
                    let thread = threads.get_mut(&first_char).unwrap();
                    let thread_string = &haystack[thread.thread_start..];
                    let thread_offset = diff_pos(thread_string, rem_suffix);
                    let (node_index, node) = thread.nodes.get_or_default_mut(thread_offset);
                    if thread_offset == rem_suffix.len() {
                        // During creation there will never be a needle
                        // that is longer than the thread. So the needle ends in this thread
                        // and there is no need to create a new thread.
                        // There can only be one suffix ending here, because suffixes all have
                        // different lengths.
                        node.contained_suffix_start = Some(suffix_start);
                        break;
                    } else {
                        // The needle goes into another thread
                        thread_start += thread_offset;
                        node_index
                    }
                } else {
                    // Creating a new thread for this needle.
                    threads.insert(first_char, Thread {
                        suffix_start,
                        thread_start,
                        nodes: SortedVec::new(),
                    });
                    break;
                };
                threads = &mut {threads}.get_mut(&first_char).unwrap()
                    .nodes.entry_at(node_index).1.branches;
            }
        }

        /*for thread in st.start_threads.iter() {
            println!("{:?}", thread);
        }*/

        return st;
    }

    fn find(&self, needle: &str, start_points: &mut Vec<usize>) {
        start_points.clear();
        let mut threads = &self.start_threads;
        let mut rem_needle = needle;
        loop {
            let first_char = rem_needle.chars().next().unwrap();
            if let Some(thread) = threads.get(&first_char) {
                let thread_string = &self.haystack[thread.thread_start..];
                let thread_offset = diff_pos(thread_string, rem_needle);
                if thread_offset == rem_needle.len() {
                    add_children(thread, thread_offset, start_points);
                    return;
                } else if let Some(node) = thread.nodes.get(&thread_offset) {
                    threads = &node.branches;
                    rem_needle = &rem_needle[thread_offset..];
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }
}

fn add_children(thread: &Thread, min_pos: usize, start_points: &mut Vec<usize>) {
    start_points.push(thread.suffix_start);
    let mut stack = Vec::new(); // TODO remove allocation
    for &(pos, ref node) in thread.nodes.iter() {
        if pos >= min_pos {
            stack.push(node);
        }
    }

    while let Some(node) = stack.pop() {
        if let Some(css) = node.contained_suffix_start {
            start_points.push(css);
        }
        for t in node.branches.values() {
            start_points.push(t.suffix_start);
            stack.extend(t.nodes.iter().map(|(_, v)| v));
        }
    }
}

fn diff_pos(a: &str, b: &str) -> usize {
    a.bytes().zip(b.bytes()).take_while(|(a, b)| a == b).count()
}
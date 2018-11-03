mod find;
mod linear_search;
mod suffix_tree;
mod sorted_vec;

extern crate rand;
extern crate reqwest;

use rand::Rng;

use find::Find;
use linear_search::LinearSearch;
use suffix_tree::SuffixTree;

fn main() {
    let text = generate_random_string(99999);
    //println!("{}", text);
    //let text = "abacbacb";
    //let text = "asdfaafdsfa";
    //let text = "";
    //let text = reqwest::get("https://www.ccel.org/ccel/bible/kjv.txt").unwrap().text().unwrap();
    //let text = reqwest::get("https://longestjokeintheworld.com/").unwrap().text().unwrap();

    //let needles = vec!["Nate", "the"];
    /*let needles_string = reqwest::get("https://raw.githubusercontent.com/dwyl/english-words/master/words.txt")
        .unwrap().text().unwrap();
    let needles: Vec<&str> = needles_string.split('\n').collect();*/
    //let needles = vec!["and the"];
    let needles = vec!["ab"];

    let ls = LinearSearch::new(&text);

    let mut ls_start_points = Vec::new();
    for needle in &needles {
        ls.find(needle, &mut ls_start_points);
        if !ls_start_points.is_empty() {
            println!("{}: {:?}", needle, ls_start_points);
        }
    }

    let st = SuffixTree::new(&text);

    let mut st_start_points = Vec::new();
    for needle in &needles {
        st.find(needle, &mut st_start_points);
        st_start_points.sort_unstable();
        if !st_start_points.is_empty() {
            println!("{}: {:?}", needle, st_start_points);
        }
    }
    assert_eq!(st_start_points, ls_start_points);
}

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

fn generate_random_string(size: usize) -> String {
    let mut result = String::new();
    for _ in 0..size {
        let num = rand::thread_rng().gen_range(0, ALPHABET.len());
        result.push(ALPHABET[num]);
    }
    result
}
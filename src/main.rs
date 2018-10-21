mod find;
mod linear_search;
mod suffix_tree;

extern crate rand;
extern crate reqwest;

use rand::Rng;

use find::Find;
use linear_search::LinearSearch;
use suffix_tree::SuffixTree;

fn main() {
    //let text = generate_random_string(999);
    //let text = "asdfaafdsa";
    //let text = "";
    let text = reqwest::get("https://longestjokeintheworld.com/").unwrap().text().unwrap();
    let needle = "Nate";
    //println!("{}", text);

    let mut ls_start_points = Vec::new();
    let ls = LinearSearch::new(&text);
    ls.find(&needle, &mut ls_start_points);
    println!("{:?}", ls_start_points);

    let mut st_start_points = Vec::new();
    let st = SuffixTree::new(&text);
    st.find(&needle, &mut st_start_points);
    println!("{:?}", st_start_points);
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
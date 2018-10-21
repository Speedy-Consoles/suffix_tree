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
    //println!("{}", text);

    //let text = reqwest::get("https://www.ccel.org/ccel/bible/kjv.txt").unwrap().text().unwrap();
    let text = reqwest::get("https://longestjokeintheworld.com/").unwrap().text().unwrap();

    //let needles = vec!["Nate", "the"];
    let needles_string = reqwest::get("https://raw.githubusercontent.com/dwyl/english-words/master/words.txt")
        .unwrap().text().unwrap();
    let needles: Vec<&str> = needles_string.split('\n').collect();

    //let needles = vec!["and the"];

    let mut start_points = Vec::new();

    /*let ls = LinearSearch::new(&text);
    for needle in &needles {
        ls.find(needle, &mut start_points);
        if !start_points.is_empty() {
            println!("{}: {:?}", needle, start_points.len());
        }
    }*/

    let st = SuffixTree::new(&text);
    for needle in &needles {
        st.find(needle, &mut start_points);
        if !start_points.is_empty() {
            println!("{}: {:?}", needle, start_points.len());
        }
    }
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
pub trait Find<'a> {
    fn new(haystack: &'a str) -> Self;
    fn find(&self, needle: &str, start_points: &mut Vec<usize>);
}
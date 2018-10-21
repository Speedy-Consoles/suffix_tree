use find::Find;

pub struct LinearSearch<'a> {
    haystack: &'a str,
}

impl<'a> Find<'a> for LinearSearch<'a> {
    fn new(haystack: &'a str) -> Self {
        LinearSearch {
            haystack,
        }
    }

    fn find(&self, needle: &str, start_points: &mut Vec<usize>) {
        let mut hcs = self.haystack.char_indices();
        loop {
            let mut next_hcs = hcs.clone();
            let current_start = match next_hcs.next() {
                Some(e) => e.0,
                None => return,
            };
            let mut found = true;
            for nc in needle.chars() {
                if let Some ((_, hc)) = hcs.next() {
                    if hc != nc {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                start_points.push(current_start);
            }
            hcs = next_hcs;
        }
    }
}
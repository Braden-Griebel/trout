use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

/// Represents a line of utf-8 encoded text
#[derive(Debug, Clone)]
pub struct Line {
    /// The text being represented
    pub(crate) text: String,
    /// How many graphemes are present in the text
    pub grapheme_count: usize,
    /// The start byte for graphemes in the text
    grapheme_starts: Vec<usize>,
    /// The end byte for graphemes in the text
    grapheme_ends: Vec<usize>,
}


impl Line {
    pub fn from_string(in_string: &str) -> Line {
        let mut grapheme_count: usize = 0;
        let mut grapheme_starts: Vec<usize> = Vec::new();
        let mut grapheme_ends: Vec<usize> = Vec::new();

        for (index, _) in UnicodeSegmentation::grapheme_indices(in_string, true) {
            grapheme_count += 1;

            // After skipping the first iteration, start adding index-1 to grapheme ends
            if grapheme_starts.len() > 0 {
                grapheme_ends.push(index.saturating_sub(1));
            }
            grapheme_starts.push(index);
        }
        // Add the end of the string to the grapheme_ends, as that is the end of the final
        // grapheme
        if grapheme_count > 0 {
            grapheme_ends.push(in_string.len().saturating_sub(1));
        }
        Line {
            text: in_string.to_string(),
            grapheme_count,
            grapheme_starts,
            grapheme_ends,
        }
    }

    /// Insert a character into the line at the specified grapheme index
    pub fn insert_char(&mut self, grapheme_index: usize, character: char) {
        // If the index is too large, panic
        if grapheme_index > self.grapheme_count {
            panic!("Tried to insert beyond end of text");
            // Check start of string case first, as this also works if the text is empty
        } else if grapheme_index == 0 {
            self.text.insert(0, character);
            let grapheme_length = character.len_utf8();
            // Increment every index following inserted character
            for idx in 0..self.grapheme_count {
                self.grapheme_starts[idx] += grapheme_length;
                self.grapheme_ends[idx] += grapheme_length;
            }
            // Insert the correct grapheme start and end
            self.grapheme_starts.insert(0, 0);
            self.grapheme_ends.insert(0, grapheme_length - 1);
            // Update grapheme count
            self.grapheme_count += 1;
        } else if grapheme_index == self.grapheme_count {
            self.text.push(character);
            let grapheme_length = character.len_utf8();
            // Update the grapheme starts and ends
            // Since its inserted at the end, only need to update the last ones
            self.grapheme_starts.push(self.grapheme_ends.last().unwrap_or(&0usize) + 1);
            self.grapheme_ends.push(self.grapheme_starts.last().unwrap_or(&0usize) + grapheme_length - 1);
            // Add one to the grapheme count
            self.grapheme_count += 1;
        } else {
            let grapheme_length = character.len_utf8();
            let text_position = self.grapheme_starts[grapheme_index];
            self.text.insert(text_position, character);
            // Update grapheme boundaries
            for idx in grapheme_index..self.grapheme_count {
                self.grapheme_starts[idx] += grapheme_length;
                self.grapheme_ends[idx] += grapheme_length;
            }
            // Insert grapheme boundaries of inserted character
            self.grapheme_starts.insert(grapheme_index, text_position);
            self.grapheme_ends.insert(grapheme_index, text_position + grapheme_length - 1);
            // Update grapheme count
            self.grapheme_count += 1;
        }
    }

    /// Insert a str into the line at the specified grapheme index
    pub fn insert_str(&mut self, grapheme_index: usize, insert_str: &str) {
        if self.grapheme_count == 0 {
            // Essentially just use the from_string method to generate a new line,
            // then copy all the properties into self
            let new_text = Line::from_string(insert_str);
            self.text = new_text.text;
            self.grapheme_ends = new_text.grapheme_ends;
            self.grapheme_starts = new_text.grapheme_starts;
            self.grapheme_count = new_text.grapheme_count;
        }
        // Use the from_string method to find the grapheme locations
        let new_text = Line::from_string(insert_str);
        let insert_idx = self.grapheme_starts[grapheme_index];
        self.text.insert_str(insert_idx, insert_str);

        // Update the grapheme indices and count
        let insert_len = match new_text.grapheme_ends.last(){
            None => {0usize}
            Some(v)=> v+1usize
        };
        for idx in grapheme_index..self.grapheme_count{
            self.grapheme_starts[idx]+=insert_len;
            self.grapheme_ends[idx]+=insert_len;
        }

        let insert_starts:Vec<usize> = new_text.grapheme_starts.iter().map(|x| {x+insert_idx}).collect();
        let insert_ends:Vec<usize> = new_text.grapheme_ends.iter().map(|x| {x+insert_idx}).collect();
        self.grapheme_starts.splice(grapheme_index..grapheme_index, insert_starts);
        self.grapheme_ends.splice(grapheme_index..grapheme_index, insert_ends);
        self.grapheme_count += new_text.grapheme_count;
    }

    /// Delete the grapheme at the specified index
    pub fn delete_grapheme(&mut self, grapheme_index: usize) {
        // If the grapheme_index is too large, don't try to delete anything
        if grapheme_index >= self.grapheme_count {
            return;
        }
        self.text.replace_range(self.grapheme_starts[grapheme_index]..=self.grapheme_ends[grapheme_index], "");
        let grapheme_length = (self.grapheme_ends[grapheme_index] - self.grapheme_starts[grapheme_index]) + 1;
        for idx in grapheme_index..self.grapheme_count {
            self.grapheme_starts[idx] -= grapheme_length;
            self.grapheme_ends[idx] -= grapheme_length;
        }
        self.grapheme_starts.remove(grapheme_index);
        self.grapheme_ends.remove(grapheme_index);
        self.grapheme_count -= 1;
    }

    pub fn grapheme_start(&self, grapheme_index: usize) -> usize {
        if self.grapheme_count == 0 {
            return 0;
        }
        if grapheme_index >= self.grapheme_count {
            return self.grapheme_starts[self.grapheme_count - 1];
        }
        self.grapheme_starts[grapheme_index]
    }

    pub fn grapheme_end(&self, grapheme_index: usize) -> usize {
        if self.grapheme_count == 0 {
            return 0;
        }
        self.grapheme_ends[grapheme_index]
    }

    pub fn next_grapheme_start(&self, grapheme_index: usize) -> usize {
        if self.grapheme_count == 0 {
            return 0;
        }
        // If the grapheme index is too large, return the last possible grapheme start instead
        if grapheme_index >= self.grapheme_count {
            return self.grapheme_starts[self.grapheme_count - 1];
        }
        self.grapheme_starts[grapheme_index + 1]
    }

    pub fn prev_grapheme_start(&self, grapheme_index: usize) -> usize {
        if self.grapheme_count == 0 {
            return 0;
        }
        // If the grapheme index is too small, return the first grapheme start instead
        if grapheme_index <= 0 {
            return self.grapheme_starts[0];
        }
        self.grapheme_starts[grapheme_index - 1]
    }

    pub fn next_grapheme_end(&self, grapheme_index: usize) -> usize {
        if self.grapheme_count == 0 {
            return 0;
        }
        // if the grapheme index is too large, return the last grapheme index instead
        if grapheme_index >= self.grapheme_count {
            return self.grapheme_ends[self.grapheme_count - 1];
        }
        self.grapheme_ends[grapheme_index + 1]
    }

    pub fn prev_grapheme_end(&self, grapheme_index: usize) -> usize {
        if self.grapheme_count == 0 {
            return 0;
        }
        if grapheme_index <= 0 {
            return self.grapheme_ends[0];
        }
        self.grapheme_ends[grapheme_index - 1]
    }

    pub fn text_index_to_grapheme_range(&mut self, text_index: usize) -> Range<usize> {
        if self.grapheme_count == 0 {
            return 0..0;
        }
        if text_index > self.text.len() {
            return self.grapheme_starts[0]..self.grapheme_ends[0];
        }
        for idx in 0..self.grapheme_count {
            if self.grapheme_ends[idx] >= text_index &&
                self.grapheme_starts[idx] <= text_index {
                return self.grapheme_starts[idx]..(self.grapheme_ends[idx] + 1);
            }
        }
        // If the above doesn't find the position, the text index is too large
        // Just return the range for the last grapheme
        self.grapheme_starts[self.grapheme_count - 1]..(self.grapheme_ends[self.grapheme_count - 1] + 1)
    }

    pub fn text_index_to_grapheme(&self, text_index: usize) -> usize {
        if self.grapheme_count == 0 {
            return 0;
        }
        if text_index > self.text.len() {
            return self.grapheme_count - 1;
        }
        for idx in 0..self.grapheme_count {
            if self.grapheme_ends[idx] >= text_index && self.grapheme_starts[idx] <= text_index {
                return idx;
            }
        }
        return self.grapheme_count - 1;
    }

    /// Split a string at the provided index. Truncates text to be the string up to that index,
    /// and returns the remainder of the text as a String.
    pub fn split_line(&mut self, index: usize) -> Line {
        let mut end_str = String::new();
        self.text[index..].clone_into(&mut end_str);
        self.text.truncate(index);
        return Line::from_string(&end_str);
    }

    /// Split a string at the provided grapheme (from the start of the grapheme)
    pub fn split_line_grapheme(&mut self, grapheme_index: usize) -> Line {
        self.split_line(self.grapheme_starts[grapheme_index])
    }
}

impl PartialEq<Self> for Line {
    fn eq(&self, other: &Self) -> bool {
        (self.text == other.text) &&
            (self.grapheme_count == self.grapheme_count) &&
            (self.grapheme_ends == other.grapheme_ends) &&
            (self.grapheme_starts == other.grapheme_starts)
    }
}

impl Eq for Line {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_ascii_str() {
        let result = Line::from_string("abcdef");
        assert_eq!(result, Line {
            text: "abcdef".to_string(),
            grapheme_count: 6usize,
            grapheme_starts: vec![0, 1, 2, 3, 4, 5],
            grapheme_ends: vec![0, 1, 2, 3, 4, 5],
        })
    }

    #[test]
    fn from_empty_str() {
        let result = Line::from_string("");
        assert_eq!(result.text, "".to_string());
        assert_eq!(result.grapheme_count, 0);
        assert_eq!(result.grapheme_starts, Vec::new());
        assert_eq!(result.grapheme_ends, Vec::new());
    }

    #[test]
    fn read_utf8_str() {
        // Flags of Ascension Island (U+1F1E6 U+1F1E8) and
        // Wales (U+1F3F4 U+E0067 U+E0062 U+E0077 U+E006C U+E0073 U+E007F)
        let result = Line::from_string("🇦🇨🏴󠁧󠁢󠁷󠁬󠁳󠁿");
        assert_eq!(result.grapheme_count, 2);
        assert_eq!(result.text, "🇦🇨🏴󠁧󠁢󠁷󠁬󠁳󠁿")
    }

    #[test]
    fn insert_at_end() {
        let mut test_line = Line::from_string("abcdef");
        test_line.insert_char(6, 'g');
        assert_eq!(test_line.text, "abcdefg".to_string());
        assert_eq!(test_line.grapheme_ends[6], 6);
        assert_eq!(test_line.grapheme_starts[6], 6);
        assert_eq!(test_line.grapheme_count, 7);
        let mut test_line = Line::from_string("€£ế");
        test_line.insert_char(3, '𐍈');
        assert_eq!(test_line.text, "€£ế𐍈"); // lengths are 3 2 3 4
        assert_eq!(test_line.grapheme_starts, vec![0, 3, 5, 8]);
        assert_eq!(test_line.grapheme_ends, vec![2, 4, 7, 11])
    }

    #[test]
    fn insert_middle() {
        let mut test_line = Line::from_string("abcdef");
        test_line.insert_char(2, 'x');
        assert_eq!(test_line.text, "abxcdef".to_string());
        assert_eq!(test_line.grapheme_ends, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(test_line.grapheme_starts, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(test_line.grapheme_count, 7);
        let mut test_line = Line::from_string("€£ế");
        test_line.insert_char(2, '𐍈');
        assert_eq!(test_line.text, "€£𐍈ế"); // lengths are 3 2 4 3
        assert_eq!(test_line.grapheme_starts, vec![0, 3, 5, 9]);
        assert_eq!(test_line.grapheme_ends, vec![2, 4, 8, 11])
    }

    #[test]
    fn insert_start() {
        let mut test_line = Line::from_string("abcdef");
        test_line.insert_char(0, 'x');
        assert_eq!(test_line.text, "xabcdef".to_string());
        assert_eq!(test_line.grapheme_starts, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(test_line.grapheme_ends, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(test_line.grapheme_count, 7);
        let mut test_line = Line::from_string("€£ế");
        test_line.insert_char(0, '𐍈');
        assert_eq!(test_line.text, "𐍈€£ế"); // lengths are 4 3 2 3
        assert_eq!(test_line.grapheme_starts, vec![0, 4, 7, 9]);
        assert_eq!(test_line.grapheme_ends, vec![3, 6, 8, 11]);
        let mut test_line = Line::from_string("");
        test_line.insert_char(0, 'a');
        assert_eq!(test_line.text, "a");
        assert_eq!(test_line.grapheme_count, 1);
        assert_eq!(test_line.grapheme_starts, vec![0]);
        assert_eq!(test_line.grapheme_ends, vec![0]);
    }

    #[test]
    fn delete_grapheme() {
        let mut test_line = Line::from_string("abcdef");
        test_line.delete_grapheme(2);
        assert_eq!(test_line.text, "abdef".to_string());
        assert_eq!(test_line.grapheme_count, 5);
        assert_eq!(test_line.grapheme_starts, vec![0, 1, 2, 3, 4]);
        assert_eq!(test_line.grapheme_ends, vec![0, 1, 2, 3, 4]);
        let mut test_line = Line::from_string("€£ế");
        test_line.delete_grapheme(1);
        assert_eq!(test_line.text, "€ế");
        assert_eq!(test_line.grapheme_count, 2);
        assert_eq!(test_line.grapheme_starts, vec![0, 3]);
        assert_eq!(test_line.grapheme_ends, vec![2, 5]);
    }

    #[test]
    fn split_line() {
        let mut test_line = Line::from_string("abcdef");
        let end_of_line = test_line.split_line(3);
        assert_eq!(test_line.text, "abc".to_string());
        assert_eq!(end_of_line.text, "def".to_string());
    }

    #[test]
    fn split_line_at_grapheme() {
        let mut test_line = Line::from_string("€£ế");
        let end_of_line = test_line.split_line_grapheme(1);
        assert_eq!(test_line.text, "€".to_string());
        assert_eq!(end_of_line.text, "£ế".to_string());
    }

    #[test]
    fn text_index_to_grapheme_range() {
        let mut test_line = Line::from_string("€£𐍈ế"); // lengths are 3 2 4 3
        let grapheme_range = test_line.text_index_to_grapheme_range(7);
        assert_eq!(grapheme_range.start, 5);
        assert_eq!(grapheme_range.end, 9);
    }
    #[test]
    fn text_index_to_grapheme() {
        let mut test_line = Line::from_string("€£𐍈ế"); // lengths are 3 2 4 3
        let grapheme = test_line.text_index_to_grapheme(7);
        assert_eq!(grapheme, 2);
    }

    #[test]
    fn insert_ascii_str(){
        let mut test_line = Line::from_string("abcdef");
        test_line.insert_str(3, "xyz");
        assert_eq!(test_line.text, "abcxyzdef".to_string());
        assert_eq!(test_line.grapheme_count, 9);
        assert_eq!(test_line.grapheme_starts, vec![0,1,2,3,4,5,6,7,8]);
        assert_eq!(test_line.grapheme_ends, vec![0,1,2,3,4,5,6,7,8])
    }

    #[test]
    fn insert_utf8_str(){
        let mut test_line = Line::from_string("€ế"); // Lengths are 3 3
        test_line.insert_str(1, "£𐍈");
        assert_eq!(test_line.text, "€£𐍈ế");
        assert_eq!(test_line.grapheme_count, 4);
        assert_eq!(test_line.grapheme_starts, vec![0,3, 5, 9]); // lengths are 3 2 4 3
        assert_eq!(test_line.grapheme_ends, vec![2, 4, 8, 11]);
    }
}
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

/// Represents a line of utf-8 encoded text
#[derive(Debug, Clone)]
pub(super) struct Line {
    /// The text being represented
    text: String,
    /// How many graphemes are present in the text
    grapheme_count: usize,
    /// The start byte for graphemes in the text
    grapheme_starts: Vec<usize>,
    /// The end byte for graphemes in the text
    grapheme_ends: Vec<usize>,
}


impl Line {
    pub fn from_string(in_string: &str) -> Line {
        let mut grapheme_count:usize = 0;
        let mut grapheme_starts: Vec<usize> = Vec::new();
        let mut grapheme_ends: Vec<usize> = Vec::new();

        for (index, _) in UnicodeSegmentation::grapheme_indices(in_string, true){
            grapheme_count += 1;

            // After skipping the first iteration, start adding index-1 to grapheme ends
            if grapheme_starts.len()>0 {
                grapheme_ends.push(index.saturating_sub(1));
            }
            grapheme_starts.push(index);
        }
        // Add the end of the string to the grapheme_ends, as that is the end of the final
        // grapheme
        grapheme_ends.push(in_string.len().saturating_sub(1));
        Line {
            text: in_string.to_string(),
            grapheme_count,
            grapheme_starts,
            grapheme_ends
        }
    }

    /// Insert a character into the line at the specified grapheme index
    pub fn insert_char(&mut self, grapheme_index: usize, character: char){
        // If the index is too large, panic
        if grapheme_index > self.grapheme_count {
            panic!("Tried to insert beyond ");
        // Otherwise, if inserting at end push to the string
        } else if grapheme_index == self.grapheme_count {
            self.text.push(character);
            // Update the grapheme starts and ends
            // Since its inserted at the end, only need to update the last ones
            self.grapheme_starts.push(self.grapheme_count);
            self.grapheme_ends.push(self.grapheme_count);
            // Add one to the grapheme count
            self.grapheme_count +=1;
        } else if grapheme_index ==0 {
            self.text.insert(0, character);
            // Increment every index following inserted character
            for idx in 0..self.grapheme_count{
                self.grapheme_starts[idx] += 1;
                self.grapheme_ends[idx] +=1;
            }
            // Insert the correct grapheme start and end
            self.grapheme_starts.insert(0,0);
            self.grapheme_ends.insert(0,0);
            // Update grapheme count
            self.grapheme_count+=1;
        } else {
            self.text.insert(self.grapheme_ends[grapheme_index], character);
            let grapheme_position = self.grapheme_ends[grapheme_index-1]+1;
            // Update grapheme boundaries
            for idx in grapheme_index..self.grapheme_count{
                self.grapheme_starts[idx]+=1;
                self.grapheme_ends[idx]+=1;
            }
            // Insert grapheme boundaries of inserted character

            self.grapheme_starts.insert(grapheme_index, grapheme_position);
            self.grapheme_ends.insert(grapheme_index, grapheme_position);
            // Update grapheme count
            self.grapheme_count+=1;
        }
    }

    /// Delete the grapheme at the specified index
    pub fn delete_grapheme(&mut self, grapheme_index:usize){
        // If the grapheme_index is too large, don't try to delete anything
        if grapheme_index>=self.grapheme_count{
            return;
        }
        self.text.replace_range(self.grapheme_starts[grapheme_index]..=self.grapheme_ends[grapheme_index], "");
        let grapheme_length = self.grapheme_ends[grapheme_index]-self.grapheme_starts[grapheme_index];
        for idx in grapheme_index..self.grapheme_count{
            self.grapheme_starts[idx] -= grapheme_length;
            self.grapheme_ends[idx] -= grapheme_length;
        }
        self.grapheme_count-=1;
    }

    pub fn grapheme_start(&mut self, grapheme_index:usize)->usize{
        if self.grapheme_count == 0 {
            return 0;
        }
        if grapheme_index>=self.grapheme_count {
            return self.grapheme_starts[self.grapheme_count-1];
        }
        self.grapheme_starts[grapheme_index]
    }

    pub fn grapheme_end(&mut self, grapheme_index: usize)->usize{
        if self.grapheme_count == 0 {
            return 0;
        }
        self.grapheme_ends[grapheme_index]
    }

    pub fn next_grapheme_start(&mut self, grapheme_index: usize)-> usize{
        if self.grapheme_count == 0 {
            return 0;
        }
        // If the grapheme index is too large, return the last possible grapheme start instead
        if grapheme_index >= self.grapheme_count {
            return self.grapheme_starts[self.grapheme_count-1]
        }
        self.grapheme_starts[grapheme_index+1]
    }

    pub fn prev_grapheme_start(&mut self, grapheme_index:usize)-> usize{
        if self.grapheme_count == 0 {
            return 0;
        }
        // If the grapheme index is too small, return the first grapheme start instead
        if grapheme_index<=0{
            return self.grapheme_starts[0]
        }
        self.grapheme_starts[grapheme_index-1]
    }

    pub fn next_grapheme_end(&mut self, grapheme_index:usize)->usize{
        if self.grapheme_count == 0 {
            return 0;
        }
        // if the grapheme index is too large, return the last grapheme index instead
        if grapheme_index>=self.grapheme_count{
            return self.grapheme_ends[self.grapheme_count-1];
        }
        self.grapheme_ends[grapheme_index+1]
    }

    pub fn prev_grapheme_end(&mut self, grapheme_index:usize)->usize{
        if self.grapheme_count == 0 {
            return 0;
        }
        if grapheme_index<=0 {
            return self.grapheme_ends[0];
        }
        self.grapheme_ends[grapheme_index-1]
    }

    pub fn text_index_to_grapheme_range(&mut self, text_index: usize)->Range<usize>{
        if self.grapheme_count == 0 {
            return 0..0;
        }
        if text_index > self.text.len() {
            return self.grapheme_starts[0]..self.grapheme_ends[0];
        }
        for idx in 0..self.grapheme_count {
            if self.grapheme_starts[idx] <= text_index {
                return self.grapheme_starts[idx]..self.grapheme_ends[idx];
            }
        }
        // If the above doesn't find the position, the text index is too large
        // Just return the range for the last grapheme
        self.grapheme_starts[self.grapheme_count-1]..self.grapheme_ends[self.grapheme_count-1]
    }
}

impl PartialEq<Self> for Line {
    fn eq(&self, other: &Self) -> bool {
        (self.text == other.text) &&
            (self.grapheme_count == self.grapheme_count) &&
            (self.grapheme_ends==other.grapheme_ends) &&
            (self.grapheme_starts == other.grapheme_starts)
    }
}

impl Eq for Line {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_ascii_str(){
        let result = Line::from_string("abcdef");
        assert_eq!(result, Line {
            text: "abcdef".to_string(),
            grapheme_count: 6usize,
            grapheme_starts: vec![0,1,2,3,4,5],
            grapheme_ends: vec![0,1,2,3,4,5],
        })
    }

    #[test]
    fn read_utf8_str(){
        // Flags of Ascension Island (U+1F1E6 U+1F1E8) and
        // Wales (U+1F3F4 U+E0067 U+E0062 U+E0077 U+E006C U+E0073 U+E007F)
        let result = Line::from_string("🇦🇨🏴󠁧󠁢󠁷󠁬󠁳󠁿");
        assert_eq!(result.grapheme_count, 2);
        assert_eq!(result.text, "🇦🇨🏴󠁧󠁢󠁷󠁬󠁳󠁿")
    }

    #[test]
    fn insert_at_end(){
        let mut test_line = Line::from_string("abcdef");
        test_line.insert_char(6, 'g');
        assert_eq!(test_line.text, "abcdefg".to_string());
        assert_eq!(test_line.grapheme_ends[6], 6);
        assert_eq!(test_line.grapheme_starts[6],6);
        assert_eq!(test_line.grapheme_count, 7);
    }

    #[test]
    fn insert_middle(){
        let mut test_line = Line::from_string("abcdef");
        test_line.insert_char(2, 'x');
        assert_eq!(test_line.text, "abxcdef".to_string());
        assert_eq!(test_line.grapheme_ends, vec![0,1,2,3,4,5,6]);
        assert_eq!(test_line.grapheme_starts, vec![0,1,2,3,4,5,6]);
        assert_eq!(test_line.grapheme_count, 7);
    }

    #[test]
    fn insert_start(){
        let mut test_line = Line::from_string("abcdef");
        test_line.insert_char(0, 'x');
        assert_eq!(test_line.text, "xabcdef".to_string());
        assert_eq!(test_line.grapheme_starts, vec![0,1,2,3,4,5,6]);
        assert_eq!(test_line.grapheme_ends, vec![0,1,2,3,4,5,6]);
        assert_eq!(test_line.grapheme_count, 7);
    }

    #[test]
    fn delete_grapheme(){
        let mut test_line = Line::from_string("abcdef");
        test_line.delete_grapheme(2);
        assert_eq!(test_line.text, "abdef".to_string());
        assert_eq!(test_line.grapheme_count, 5);
        assert_eq!(test_line.grapheme_starts, vec![0,1,2,3,4,5]);
        assert_eq!(test_line.grapheme_ends, vec![0,1,2,3,4,5]);
    }
}
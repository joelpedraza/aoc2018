use super::bitvec::BitVec32;

// A 26 way Trie based on Array Mapped Trees "Fast And Space Efficient Trie Searches" - Bagwell 2000
// Bits 0-25 are used as the AMT mask
// Bit 26 is the leaf flag
// Bits 27-32 are unused

/// A ... Z is 26 letters
const ALPHA_CHAR_COUNT: usize = 26;

// BIT FLAGS

/// The flag indicating a node is a leaf
const LEAF_FLAG_INDEX: usize = 26;

/// A simple Trie for ascii alpha characters with a branching factor of 26.
///
/// Only supports insert.
#[derive(Debug)]
pub struct Trie {
    len: usize,
    root: Node,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            len: 0,
            // init the root with enough space to hold all children
            root: Node::with_capacity(ALPHA_CHAR_COUNT),
        }
    }

    /// The number of strings in the trie
    pub fn len(&self) -> usize {
        self.len
    }

    /// Insert a string into the trie
    ///
    /// If the set did not have this value present, [true] is returned.
    /// If the set did have this value present, [false] is returned.
    pub fn insert(&mut self, s: &str) -> bool {
        let inserted = self.root.insert_search(s);

        if inserted {
            self.len += 1;
        }

        inserted
    }

    /// Checks whether or not [s] is in the trie
    ///
    /// If the set has this value present, [true] is returned.
    /// If the set does have this value present, [false] is returned.
    pub fn contains(&mut self, s: &str) -> bool {
        self.root.contains(s)
    }

    /// Gets the given string's corresponding entry in the trie for in-place manipulation.
    pub fn entry(&mut self, s: &str) -> Option<Entry> {
        self.root.find_entry(0, s)
    }
}


// Node
// ====

#[derive(Debug)]
struct Node {
    mask: BitVec32,
    children: Vec<Node>,
}

impl Node {
    fn new() -> Node {
        Self::with_capacity(0)
    }

    fn with_capacity(capacity: usize) -> Node {
        Node {
            mask: BitVec32::new(),
            children: Vec::with_capacity(capacity),
        }
    }

    fn is_leaf(&self) -> bool {
        self.mask.get(LEAF_FLAG_INDEX)
    }

    fn set_leaf(&mut self) {
        self.mask.set(LEAF_FLAG_INDEX)
    }

    /// The prefix [s] may be in the trie, walk recursively to look for an insertion point
    /// return true if we performed an insertion
    fn insert_search(&mut self, s: &str) -> bool {
        match s.chars()
            .next()
            .as_ref()
            .map(char_to_index) {
            Some(char_idx) => {
                let is_present = self.mask.get(char_idx);
                let tail = &s[1..];

                if is_present {
                    let index = index_to_bit_pos(&self.mask, char_idx);
                    self.children[index].insert_search(tail)
                } else {
                    self.insert(s);
                    true
                }
            }

            None => {
                let is_leaf = self.is_leaf();
                self.set_leaf();
                !is_leaf
            }
        }
    }

    /// The prefix [s] may be in the trie, walk recursively to look for an insertion point
    /// return true if we performed an insertion
    fn find_entry(&mut self, depth: usize, s: &str) -> Option<Entry> {
        match s.chars()
            .next()
            .as_ref()
            .map(char_to_index) {
            Some(char_idx) => {
                let is_present = self.mask.get(char_idx);
                let tail = &s[1..];

                if is_present {
                    let index = index_to_bit_pos(&self.mask, char_idx);
                    self.children[index].find_entry(depth + 1, tail)
                } else {
                    Some(Entry {
                        node: self,
                        index: depth,
                    })
                }
            }

            None => None
        }
    }

    /// The prefix [s] has been guaranteed to be absent (because a new node was created)
    /// perform insertion recursively without checking for node presence
    fn insert(&mut self, s: &str) {
        match s.chars()
            .next()
            .as_ref()
            .map(char_to_index) {
            Some(char_idx) => {
                let tail = &s[1..];
                let index = index_to_bit_pos(&self.mask, char_idx);

                debug_assert!(!self.mask.get(char_idx), "found a child during insert");

                if index == self.children.len() {
                    self.children.push(Node::new());
                } else {
                    self.children.insert(index, Node::new());
                }

                self.mask.set(char_idx);

                self.children[index].insert(tail)
            }

            None => {
                self.set_leaf();
            }
        }
    }

    fn contains(&self, s: &str) -> bool {
        match s.chars()
            .next()
            .as_ref()
            .map(char_to_index)
            {
                Some(char_idx) => {
                    let is_present = self.mask.get(char_idx);

                    if is_present {
                        let index = index_to_bit_pos(&self.mask, char_idx);
                        self.children[index].contains(&s[1..])
                    } else {
                        false
                    }
                }
                None => self.is_leaf()
            }
    }
}

pub struct Entry<'a> {
    node: &'a mut Node,
    index: usize,
}

impl<'a> Entry<'a> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn contains(&self, tail: &str) -> bool {
        self.node.contains(tail)
    }

    pub fn insert(&mut self, tail: &str) -> bool {
        self.node.insert_search(tail)
    }

    /*pub fn iter(&self) -> EntryIter {
        EntryIter {
            iter: self.node.children.iter(),
            index: self.index + 1,
        }
    }*/

    pub fn iter_mut(&mut self) -> EntryIterMut {
        EntryIterMut {
            iter: self.node.children.iter_mut(),
            index: self.index + 1,
        }
    }
}

/*struct EntryIter<'a> {
    iter: std::slice::Iter<'a, Node>,
    index: usize,
}*/

/*impl<'a> Iterator for EntryIter<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .map(|node| {
                Entry {
                    node: node,
                    index: self.index,
                }
            })
    }
}*/

pub struct EntryIterMut<'a> {
    iter: std::slice::IterMut<'a, Node>,
    index: usize,
}

impl<'a> Iterator for EntryIterMut<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .map(|mut node| {
                Entry {
                    node: node,
                    index: self.index,
                }
            })
    }
}


// HELPERS
// =======

fn char_to_index(c: &char) -> usize {
    match *c {
        'a'...'z' => (*c as usize - 'a' as usize),
        'A'...'Z' => (*c as usize - 'A' as usize),
        _ => panic!("non alpha char: {}", c)
    }
}

fn index_to_bit_pos(bitmap: &BitVec32, i: usize) -> usize {
    let mask = (1 << i) - 1;
    (bitmap.to_u32() & mask).count_ones() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_returns_true_when_new_value_is_inserted() {
        let mut trie = Trie::new();

        assert_eq!(true, trie.insert("abcde"));
        assert_eq!(false, trie.insert("abcde"));
        assert_eq!(true, trie.insert("abcdef"));
        assert_eq!(false, trie.insert("abcdef"));
    }

    #[test]
    fn insert_returns_true_when_new_value_is_inserted_2() {
        let mut trie = Trie::new();

        assert_eq!(true, trie.insert("abcd"));

        // prefix is present, but is not leaf
        assert_eq!(true, trie.insert("abc"));
        assert_eq!(false, trie.insert("abc"));
    }

    #[test]
    fn contains() {
        let mut trie = Trie::new();

        assert_eq!(false, trie.contains("abc"));

        trie.insert("abc");
        assert_eq!(true, trie.contains("abc"));
        assert_eq!(false, trie.contains("ab"));
    }

    #[test]
    fn len() {
        let mut trie = Trie::new();

        assert_eq!(0, trie.len());

        trie.insert("abc");
        assert_eq!(1, trie.len());

        trie.insert("abd");
        assert_eq!(2, trie.len());

        trie.insert("abc");
        assert_eq!(2, trie.len());

        trie.insert("ab");
        assert_eq!(3, trie.len());
    }

    #[test]
    fn lookup_bit_pos() {
        let mut bv = BitVec32::new();
        assert_eq!(0, index_to_bit_pos(&bv, 0));

        bv.set(0);
        assert_eq!(1, index_to_bit_pos(&bv, 1));

        bv.set(15);
        assert_eq!(1, index_to_bit_pos(&bv, 1));
        assert_eq!(1, index_to_bit_pos(&bv, 1));
        assert_eq!(1, index_to_bit_pos(&bv, 15));
        assert_eq!(2, index_to_bit_pos(&bv, 17));

        bv.set(1);
        assert_eq!(2, index_to_bit_pos(&bv, 14));
        assert_eq!(3, index_to_bit_pos(&bv, 17));

        let bv = BitVec32::ALL;
        assert_eq!(0, index_to_bit_pos(&bv, 0));
        assert_eq!(1, index_to_bit_pos(&bv, 1));
        assert_eq!(15, index_to_bit_pos(&bv, 15));
        assert_eq!(31, index_to_bit_pos(&bv, 31));
    }

    #[test]
    fn entry_is_found() {
        let mut trie = Trie::new();
        trie.insert("fghij");

        let mut entry = trie.entry("fguij").unwrap();

        assert_eq!(2, entry.index());

        let entries: Vec<Entry> = entry.iter_mut().collect();
        assert_eq!(1, entries.len());
        assert_eq!(true, entries[0].contains("ij"));
    }
}
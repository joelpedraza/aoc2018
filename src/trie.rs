use super::bitvec::BitVec32;

const ALPHA_CHAR_COUNT: usize = 26; // A ... Z

/// A simple Trie for ascii alpha characters with a branching factor of 26.
///
/// Only supports insert.
/// Only holds prefixes, not words. (Does not currently flag leaves)
pub struct Trie {
    len: usize,
    root: Node
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            len: 0,
            // init the root with enough space to hold all children
            root: Node::with_capacity(ALPHA_CHAR_COUNT)
        }
    }

    /// Insert a string into the trie, meanwhile search for a string with a single transposed char
    ///
    /// return [Some(index)] of the transposed character if the trie contains a prefix equal to
    /// [s] but with only a single character transposed
    ///
    // TODO This is a a contrived use case!
    // Implement something more like HashMap.Entry to allow consumers of this api to find the
    // insertion point
    pub fn insert_search_single_transpose(&mut self, s: &str) -> Option<usize> {
        self.len += 1;
        self.root.insert_search_transpose(s)
            .map(|tail_len| {
                s.len() - tail_len
            })
    }

    /// Insert a string into the trie
    ///
    /// If the set did not have this value present, [true] is returned.
    /// If the set did have this value present, [false] is returned.
    // TODO when this trie is aware of leaves, this will need to be updated
    pub fn insert(&mut self, s: &str) -> bool {
        self.len += 1;
        self.root.insert_search(s)
    }


}


// Node
// ====

struct Node {
    // TODO add a flag for is-leaf. Only least significant 26 bits are used.
    mask: BitVec32,
    children: Vec<Node>
}

impl Node {

    fn new() -> Node {
        Self::with_capacity(0)
    }

    fn with_capacity(capacity: usize) -> Node {
        Node {
            mask: BitVec32::new(),
            children: Vec::with_capacity(capacity)
        }
    }

    fn insert_search_transpose(&mut self, s: &str) -> Option<usize> {
        match s.chars()
            .next()
            .as_ref()
            .map(char_to_index) {

            Some(char_idx) => {
                let is_present = self.mask.get(char_idx);
                let tail = &s[1..];

                if is_present {
                    let index = index_to_bit_pos(&self.mask, char_idx);
                    self.children[index].insert_search_transpose(tail)
                } else {

                    // walk the rest of the trie after this node. if we hit a leaf the prefix
                    // is in the trie

                    let has_single_transpose = self.children.iter()
                        .filter(|x| { x.lookup(tail) })
                        .map(|_|{true})
                        .next()
                        .unwrap_or(false);

                    if has_single_transpose {
                        Some(s.len())
                    } else {
                        self.insert(s);
                        None
                    }
                }
            },

            None => None

        }
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
            },

            None => false
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
            },

            None => {}

        }
    }

    fn lookup(&self, s: &str) -> bool {
        match s.chars()
            .next()
            .as_ref()
            .map(char_to_index)
            {
                Some(char_idx) => {
                    let is_present = self.mask.get(char_idx);

                    if is_present {
                        let index = index_to_bit_pos(&self.mask, char_idx);
                        self.children[index].lookup(&s[1..])
                    } else {
                        false
                    }
                }
                None => true
            }
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
    fn insert_single_letters() {
        let mut trie = Trie::new();

        assert_eq!(false, trie.insert_search_single_transpose("a").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("b").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("c").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("d").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("e").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("f").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("g").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("h").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("i").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("j").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("k").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("l").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("m").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("n").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("o").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("p").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("p").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("q").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("r").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("s").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("t").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("u").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("v").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("w").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("x").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("y").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("z").is_some());
    }

    #[test]
    fn insert_returns_true_when_new_value_is_inserted() {
        let mut trie = Trie::new();

        assert_eq!(true, trie.insert("abcde"));
        assert_eq!(false, trie.insert("abcde"));
        assert_eq!(true, trie.insert("abcdef"));
        assert_eq!(false, trie.insert("abcdef"));

        // TODO this should return true when the trie supports leaves
        assert_eq!(false, trie.insert("abc"));
    }

    #[test]
    fn insert_search_package_ids() {
        let mut trie = Trie::new();

        assert_eq!(false, trie.insert_search_single_transpose("abcde").is_some());
        assert_eq!(false, trie.insert_search_single_transpose("fghij").is_some());
        assert_eq!(false, trie.insert_search_single_transpose("klmno").is_some());
        assert_eq!(false, trie.insert_search_single_transpose("pqrst").is_some());
        assert_eq!(true, trie.insert_search_single_transpose("fguij").is_some());
        assert_eq!(false, trie.insert_search_single_transpose("axcye").is_some());
        assert_eq!(false, trie.insert_search_single_transpose("wvxyz").is_some());
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
}
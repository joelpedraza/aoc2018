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

    /// Insert a string into the trie
    ///
    /// return [Some(index)] of the transposed character when the trie contains a prefix equal to
    /// [s] but with only a single character transposed
    ///
    // TODO This is a very contrived definition of insert!
    // Implement something more like HashMap.Entry to allow consumers of this api to find the
    // insertion point
    pub fn insert(&mut self, s: &str) -> Option<usize> {
        self.len += 1;
        self.root.insert(0, s)
            .map(|tail_len| { s.len() - tail_len } )
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

    #[inline]
    fn insert(&mut self, transpose_count: usize, s: &str) -> Option<usize> {
        match s.chars()
            .next()
            .as_ref()
            .map(char_to_index) {

            Some(char_idx) => {
                let is_present = self.mask.get(char_idx);

                if is_present {
                    let index = index_to_bit_pos(&self.mask, char_idx);
                    self.children[index].insert(transpose_count, &s[1..])
                } else {


                    let tail = &s[1..];

                    // walk the rest of the trie after this node. if we hit a leaf the prefix
                    // is in the trie

                    if transpose_count == 0 {
                        let has_single_transpose = self.children.iter()
                            .filter(|x| { x.lookup(tail) })
                            .map(|_|{true})
                            .next()
                            .unwrap_or(false);

                        if has_single_transpose {
                            return Some(s.len())
                        }
                    }


                    // the prefix is not in the trie, keep inserting

                    let insertion_point = index_to_bit_pos(&self.mask, char_idx);

                    // self.children.reserve_exact(1);

                    if insertion_point == self.children.len() {
                        self.children.push(Node::new());
                    } else {
                        self.children.insert(insertion_point, Node::new());
                    }

                    self.mask.set(char_idx);

                    self.children[insertion_point].insert(transpose_count + 1, tail)
                }


            },

            None => None

        }
    }

    #[inline]
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
                None => {
                    true
                }
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

        assert_eq!(false, trie.insert("a").is_some());
        assert_eq!(true, trie.insert("b").is_some());
        assert_eq!(true, trie.insert("c").is_some());
        assert_eq!(true, trie.insert("d").is_some());
        assert_eq!(true, trie.insert("e").is_some());
        assert_eq!(true, trie.insert("f").is_some());
        assert_eq!(true, trie.insert("g").is_some());
        assert_eq!(true, trie.insert("h").is_some());
        assert_eq!(true, trie.insert("i").is_some());
        assert_eq!(true, trie.insert("j").is_some());
        assert_eq!(true, trie.insert("k").is_some());
        assert_eq!(true, trie.insert("l").is_some());
        assert_eq!(true, trie.insert("m").is_some());
        assert_eq!(true, trie.insert("n").is_some());
        assert_eq!(true, trie.insert("o").is_some());
        assert_eq!(true, trie.insert("p").is_some());
        assert_eq!(true, trie.insert("p").is_some());
        assert_eq!(true, trie.insert("q").is_some());
        assert_eq!(true, trie.insert("r").is_some());
        assert_eq!(true, trie.insert("s").is_some());
        assert_eq!(true, trie.insert("t").is_some());
        assert_eq!(true, trie.insert("u").is_some());
        assert_eq!(true, trie.insert("v").is_some());
        assert_eq!(true, trie.insert("w").is_some());
        assert_eq!(true, trie.insert("x").is_some());
        assert_eq!(true, trie.insert("y").is_some());
        assert_eq!(true, trie.insert("z").is_some());
    }

    #[test]
    fn insert_single_packge_ids() {
        let mut trie = Trie::new();

        assert_eq!(false, trie.insert("abcde").is_some());
        assert_eq!(false, trie.insert("fghij").is_some());
        assert_eq!(false, trie.insert("klmno").is_some());
        assert_eq!(false, trie.insert("pqrst").is_some());
        assert_eq!(true, trie.insert("fguij").is_some());
        assert_eq!(false, trie.insert("axcye").is_some());
        assert_eq!(false, trie.insert("wvxyz").is_some());
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
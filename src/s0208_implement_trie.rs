use std::collections::HashMap;

struct Node {
    children: HashMap<char, Node>,
    is_terminal: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::with_capacity(30),
            is_terminal: false,
        }
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new() }
    }

    #[inline]
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(Node::new());
        }
        node.is_terminal = true;
    }

    fn last_node(&self, word: String) -> Option<&Node> {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get_key_value(&c) {
                Some((_, node_)) => node = node_,
                None => return None,
            }
        }
        return Some(node);
    }

    #[inline]
    pub fn search(&self, word: String) -> bool {
        match self.last_node(word) {
            None => return false,
            Some(node) => return node.is_terminal,
        }
    }

    #[inline]
    pub fn starts_with(&self, prefix: String) -> bool {
        match self.last_node(prefix) {
            None => return false,
            Some(_) => return true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut trie = Trie::new();
        trie.insert(String::from("Hello"));

        assert!(trie.search(String::from("Hello")));

        assert!(!trie.search(String::from("Hello1")));
        assert!(!trie.search(String::from("Hell")));
        assert!(!trie.search(String::from("Else")));

        assert!(trie.starts_with(String::from("H")));
        assert!(trie.starts_with(String::from("Hell")));
        assert!(trie.starts_with(String::from("Hello")));
        assert!(!trie.starts_with(String::from("Else")));
    }

    #[test]
    fn insert_multiple() {
        let mut trie = Trie::new();
        trie.insert(String::from("apple"));
        assert!(trie.search(String::from("apple")));
        assert!(!trie.search(String::from("app")));
        assert!(trie.starts_with(String::from("app")));
        trie.insert(String::from("app"));
        assert!(trie.search(String::from("app")));
    }
}

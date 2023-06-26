use std::ops::{Deref, DerefMut};

use super::Pool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Digit(usize);

#[derive(Debug, Default)]
struct Node {
    next: [Option<Box<Node>>; 10],
    end: bool,
    count: usize,
}

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

#[derive(Debug, Default)]
pub struct TriePool {
    trie: Trie,
}

impl TryFrom<usize> for Digit {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if (0..=9).contains(&value) {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl Deref for Digit {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Digit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Node {
    fn new() -> Self {
        Self {
            next: [None, None, None, None, None, None, None, None, None, None],
            end: false,
            count: 0,
        }
    }

    fn extend(&mut self, digit: Digit) {
        self.next[*digit] = Some(Box::new(Node::new()))
    }

    fn shrink(&mut self, digit: Digit) {
        if self.next[*digit].is_some() && self.next[*digit].as_ref().unwrap().count == 0 {
            self.next[*digit] = None;
        }
    }
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn insert(&mut self, num: u64) -> bool {
        let digits = Self::split(num);
        Self::insert_impl(&mut self.root, &mut digits.into_iter())
    }

    pub fn remove(&mut self, num: u64) -> bool {
        let digits = Self::split(num);
        Self::remove_impl(&mut self.root, &mut digits.into_iter())
    }

    pub fn match_or_complete_suffix(&self, pattern: u64) -> Option<Vec<u64>> {
        let mut digits = Self::split(pattern);
        let mut node = &self.root;

        for &digit in &digits {
            node = node.next[*digit].as_deref()?;
        }

        if node.end {
            Some(vec![Self::concat(&digits)])
        } else {
            let mut res = Vec::new();
            Self::search_suffix(node, &mut res, &mut digits);

            if !res.is_empty() {
                Some(res)
            } else {
                None
            }
        }
    }

    fn split(num: u64) -> Vec<Digit> {
        let mut num: usize = num as usize;
        let mut digits: Vec<Digit> = Vec::with_capacity(20);

        if num == 0 {
            digits.push(0.try_into().unwrap());
        } else {
            while num != 0 {
                digits.push((num % 10).try_into().unwrap());
                num /= 10;
            }
        }

        digits.reverse();
        digits
    }

    fn insert_impl<I: Iterator<Item = Digit>>(node: &mut Node, digits: &mut I) -> bool {
        match digits.next() {
            Some(digit) => {
                if node.next[*digit].is_none() {
                    node.extend(digit);
                }

                if Self::insert_impl(node.next[*digit].as_deref_mut().unwrap(), digits) {
                    node.count += 1;
                    true
                } else {
                    false
                }
            }
            None => {
                if !node.end {
                    node.end = true;
                    node.count += 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn remove_impl<I: Iterator<Item = Digit>>(node: &mut Node, digits: &mut I) -> bool {
        match digits.next() {
            Some(digit) => {
                if node.next[*digit].is_none() {
                    false
                } else if Self::remove_impl(node.next[*digit].as_deref_mut().unwrap(), digits) {
                    node.count -= 1;
                    node.shrink(digit);
                    true
                } else {
                    false
                }
            }
            None => {
                if node.end {
                    node.end = false;
                    node.count -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn search_suffix(node: &Node, res: &mut Vec<u64>, stack: &mut Vec<Digit>) {
        if node.end {
            res.push(Self::concat(stack));
        }

        for digit in (0..10).map(|d| Digit::try_from(d).unwrap()) {
            if let Some(node) = node.next[*digit].as_deref() {
                stack.push(digit);
                Self::search_suffix(node, res, stack);
                stack.pop();
            }
        }
    }

    fn concat(digits: &[Digit]) -> u64 {
        digits.iter().fold(0, |acc, v| acc * 10 + v.0 as u64)
    }
}

impl TriePool {
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }
}

impl From<Trie> for TriePool {
    fn from(value: Trie) -> Self {
        Self { trie: value }
    }
}

impl Pool for TriePool {
    fn add(&mut self, id: u64) -> bool {
        self.trie.insert(id)
    }

    fn remove(&mut self, id: u64) -> bool {
        self.trie.remove(id)
    }

    fn find(&self, pattern: u64) -> Option<Vec<u64>> {
        self.trie.match_or_complete_suffix(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_complete_number_when_giving_prefix() {
        let mut trie = Trie::new();
        trie.insert(1234567);
        trie.insert(1234568);
        trie.insert(123457);
        trie.insert(12345);
        assert_eq!(trie.match_or_complete_suffix(12345), Some(vec![12345]));
        assert_eq!(
            trie.match_or_complete_suffix(123456),
            Some(vec![1234567, 1234568])
        );
    }
}

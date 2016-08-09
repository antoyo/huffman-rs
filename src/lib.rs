/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Huffman is a library to decode huffman-encoded data.
//!
//! # Usage
//!
//! Create a Huffman `Tree` and call the `decode` function to decode the input.

/*
 * TODO: use nom.
 * TODO: write safe code only (have a way to have a reference to a node).
 * TODO: decode should return a Result.
 * TODO: using an array of 2 Option<Tree> might be faster than having the fields left and right.
 * TODO: update the travis script to use coveralls (since travis-cargo does not work).
 */

extern crate bitreader;

use bitreader::BitReader;

/// Go deeper in the tree, following the direction specified by `$dir`.
/// When a leaf is reached, add the value to `$result` and reset the `$current_node` to the
/// `$root`.
macro_rules! visit {
    ($current_node:expr, $root:expr, $result:expr, $dir:ident) => {
        let node = unsafe { &*$current_node };
        let $dir: &Tree = node.$dir.as_ref().unwrap();

        if let Some(byte) = $dir.value {
            $result.push(byte);
            $current_node = $root as *const Tree;
        }
        else {
            $current_node = $dir as *const Tree;
        }
    };
}

/// A huffman tree.
#[derive(Default)]
pub struct Tree {
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
    pub value: Option<u8>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            left: None,
            right: None,
            value: None,
        }
    }

    pub fn new_with_value(value: u8) -> Self {
        Tree {
            left: None,
            right: None,
            value: Some(value),
        }
    }
}

/// Decode the huffman-encoded `input` using the Huffman `tree`.
/// The decoding ends when `decompressed_size` is reached.
pub fn decode(input: &[u8], tree: &Tree, decompressed_size: usize) -> Vec<u8> {
    decode_with_offset(input, 0, tree, decompressed_size)
}

/// Decode the huffman-encoded `input` using the Huffman `tree`.
/// The decoding starts at input + `offset`, where offset is the number bits to skip (number between
/// 0 and 8).
/// The decoding ends when `decompressed_size` is reached.
pub fn decode_with_offset(input: &[u8], offset: u8, tree: &Tree, decompressed_size: usize) -> Vec<u8> {
    debug_assert!(offset <= 8);
    let mut result = vec![];
    let mut reader = BitReader::new(input);

    let mut current_node = tree as *const Tree;

    // Skip the bits.
    reader.read_u8(offset).unwrap();

    while let Ok(bit) = reader.read_u8(1) {
        if result.len() == decompressed_size {
            break;
        }

        if bit == 1 {
            visit!(current_node, tree, result, right);
        }
        else {
            visit!(current_node, tree, result, left);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::u8;

    use super::{Tree, decode, decode_with_offset};

    /// Create a Huffman tree from the `bytes`.
    fn create_tree(bytes: &[u8]) -> Tree {
        let mut occurences = HashMap::new();
        for byte in bytes {
            let entry = occurences.entry(byte).or_insert(0);
            *entry += 1;
        }

        let mut nodes = BTreeMap::new();

        for (&byte, occurence) in occurences {
            nodes.insert((occurence, byte), Tree::new_with_value(byte));
        }

        let mut index = u8::MAX;

        while nodes.len() > 1 {
            let ((occurence1, _), node1) = pop_first(&mut nodes);
            let ((occurence2, _), node2) = pop_first(&mut nodes);
            let new_occurence = occurence1 + occurence2;
            let mut new_node = Tree::new();
            new_node.left = Some(Box::new(node1));
            new_node.right = Some(Box::new(node2));
            nodes.insert((new_occurence, index), new_node);
            index -= 1;
        }

        pop_first(&mut nodes).1
    }

    /// Remove the first element of the `map`.
    fn pop_first<K, V>(map: &mut BTreeMap<K, V>) -> (K, V)
        where K: Clone + Ord
    {
        let key = map.keys().next().unwrap().clone();
        let value = map.remove(&key).unwrap();
        (key, value)
    }

    #[test]
    fn test_decode() {
        let input = [0b01110010, 0b00110110, 0b11100110, 0b11011110, 0b00001111, 0b10101001, 0b10000000, 0b10101100, 0b01011111, 0b10011101, 0b11110011, 0b10010010, 0b00110111, 0b01000010, 0b00001111, 0b01110101, 0b11011010];
        let string = b"this is an example of a huffman tree";
        let tree = create_tree(string);

        let result = decode(&input, &tree, string.len());

        let expected: Vec<u8> = string.iter()
            .cloned()
            .collect();

        assert_eq!(expected, result);

        // Test that the additional bits are not used.
        let input = [0b01110010, 0b00110110, 0b11100110, 0b11011110, 0b00001111, 0b10101001, 0b10000000, 0b10101100, 0b01011111, 0b10011101, 0b11110011, 0b10010010, 0b00110111, 0b01000010, 0b00001111, 0b01110101, 0b11011010, 0b00000000];

        // Test with an offset.
        let result = decode_with_offset(&input, 4, &tree, string.len() - 1);
        assert_eq!(&expected[1..], &result[..]);
    }
}

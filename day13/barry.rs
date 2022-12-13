use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/13");

#[time_run2("13")]
fn main() {
    distress_signal(INPUT)
}

#[derive(Debug, PartialEq, Eq)]
enum Cmp {
    Ordered,
    Indetermined,
    NotOrdered,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(untagged)]
enum Element {
    Value(u64),
    List(Vec<Element>),
}

impl Element {
    // This seems to give the same results as serde::Deserialize, so this is actually fine...
    // I swapped to serde in case there was a bug in this bit of the code - turns out there was not!
    fn from_char_array(s: &[char]) -> Self {
        let mut elements: Vec<Element> = vec![];

        // Get rid of the beginning and end of the outer most brackets -- this assumes we always parse a list
        // rather than a standalone value.
        let mut rest = &s[1..s.len() - 1];

        let mut current_chars: Vec<char> = vec![];
        while !rest.is_empty() {
            match rest[0] {
                '[' => {
                    // Find the associated closed bracket.
                    let mut matching_closed_brackets_index = 0;
                    let mut matching_closed_braces = 0;

                    for (i, c) in rest[1..].iter().enumerate() {
                        // We have one more matching brace to find
                        if *c == '[' {
                            matching_closed_braces -= 1;
                        } else if *c == ']' {
                            matching_closed_braces += 1;
                            matching_closed_brackets_index = 1 + i;
                        }
                        if matching_closed_braces == 1 {
                            break;
                        }
                    }
                    debug_assert!(matching_closed_braces != 0);

                    let inner_list_elem =
                        Element::from_char_array(&rest[0..matching_closed_brackets_index + 1]);

                    elements.push(inner_list_elem);
                    rest = &rest[matching_closed_brackets_index + 1..]
                }
                ',' => {
                    if !current_chars.is_empty() {
                        let s: String = current_chars.iter().collect();
                        elements.push(Element::Value(s.parse::<u64>().unwrap()));
                        current_chars.clear();
                    }

                    rest = &rest[1..]
                }
                digit => {
                    current_chars.push(digit);
                    rest = &rest[1..]
                }
            }
        }

        if !current_chars.is_empty() {
            let s: String = current_chars.iter().collect();
            elements.push(Element::Value(s.parse::<u64>().unwrap()));
        }

        Self::List(elements)
    }

    fn is_ordered(&self, other: &Element) -> Cmp {
        match (self, other) {
            (Element::Value(self_value), Element::Value(other_value)) => {
                if self_value > other_value {
                    return Cmp::NotOrdered;
                }
                if self_value == other_value {
                    return Cmp::Indetermined;
                }
                Cmp::Ordered
            }
            (Element::Value(self_value), Element::List { .. }) => {
                let self_list = Element::List(vec![Element::Value(*self_value)]);
                self_list.is_ordered(other)
            }
            (Element::List { .. }, Element::Value(other_value)) => {
                let other_list = Element::List(vec![Element::Value(*other_value)]);
                self.is_ordered(&other_list)
            }
            (Element::List(elems_self), Element::List(elems_other)) => {
                for (i, _) in elems_self.iter().enumerate() {
                    let other = match elems_other.get(i) {
                        Some(elem) => elem,
                        None => return Cmp::NotOrdered,
                    };

                    match elems_self[i].is_ordered(other) {
                        Cmp::Ordered => return Cmp::Ordered,
                        Cmp::Indetermined => continue,
                        Cmp::NotOrdered => return Cmp::NotOrdered,
                    }
                }
                if elems_self.len() == elems_other.len() {
                    return Cmp::Indetermined;
                }
                Cmp::Ordered
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Cmp, Element};

    #[test]
    fn single_value() {
        let input: String = "[10]".chars().collect();
        let elem: Element = serde_json::from_str(&input).unwrap();
        assert_eq!(elem, Element::List(vec![Element::Value(10),]));
    }

    #[test]
    fn values() {
        let input: String = "[1,1,3,1,1]".chars().collect();
        let elem: Element = serde_json::from_str(&input).unwrap();

        assert_eq!(
            elem,
            Element::List(vec![
                Element::Value(1),
                Element::Value(1),
                Element::Value(3),
                Element::Value(1),
                Element::Value(1),
            ])
        );
    }

    #[test]
    fn single_nested() {
        let input: String = "[[1],4]".chars().collect();
        let elem: Element = serde_json::from_str(&input).unwrap();
        assert_eq!(
            elem,
            Element::List(vec![
                Element::List(vec![Element::Value(1)]),
                Element::Value(4),
            ])
        );
    }

    #[test]
    fn multiple_nested() {
        let input: Vec<char> = "[[1],[2]]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List(vec![
                Element::List(vec![Element::Value(1)]),
                Element::List(vec![Element::Value(2)]),
            ])
        );
    }

    #[test]
    fn double_nested() {
        let input: Vec<char> = "[[[1],2],3]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List(vec![
                Element::List(vec![
                    Element::List(vec![Element::Value(1)]),
                    Element::Value(2)
                ]),
                Element::Value(3)
            ])
        );
    }

    #[test]
    fn empty_nested() {
        let input: Vec<char> = "[[]]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(elem, Element::List(vec![Element::List(vec![])]));
    }

    #[test]
    fn empty_multiple_nested() {
        let input: Vec<char> = "[[[[]]],[]]".chars().collect();

        let elem = Element::from_char_array(&input);

        assert_eq!(
            elem,
            Element::List(vec![
                Element::List(vec![Element::List(vec![Element::List(vec![])])]),
                Element::List(vec![])
            ])
        );
    }

    #[test]
    fn is_ordered_simple() {
        let left: Vec<char> = "[1,2,4]".chars().collect();
        let right: Vec<char> = "[1,2,5]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::Ordered);
    }

    #[test]
    fn is_ordered_diff_size() {
        let left: Vec<char> = "[7,7,7,7]".chars().collect();
        let right: Vec<char> = "[7,7,7]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::NotOrdered);
    }

    #[test]
    fn is_ordered_nested() {
        let left: Vec<char> = "[[2,3,4]]".chars().collect();
        let right: Vec<char> = "[4]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::Ordered);
    }

    #[test]
    fn is_ordered_empty_nested() {
        let left: Vec<char> = "[[[[]]]]".chars().collect();
        let right: Vec<char> = "[[]]".chars().collect();

        let elem_l = Element::from_char_array(&left);
        let elem_r = Element::from_char_array(&right);

        assert_eq!(elem_l.is_ordered(&elem_r), Cmp::NotOrdered);
    }
}

fn distress_signal(i: &str) -> (String, String) {
    let groups = i.split("\n\n");
    let pairings: Vec<(Element, Element)> = groups
        .into_iter()
        .map(|group| {
            let v: Vec<&str> = group.split('\n').collect();
            let first: String = v[0].chars().collect();
            let second: Vec<char> = v[1].chars().collect();
            (
                serde_json::from_str::<'_, Element>(&first).unwrap(),
                Element::from_char_array(&second),
            )
        })
        .collect();

    // Part 1
    let mut ordered_indices: Vec<usize> = vec![];
    for (i, pair) in pairings.iter().enumerate() {
        if pair.0.is_ordered(&pair.1) == Cmp::Ordered {
            ordered_indices.push(i + 1)
        }
    }
    let part1 = ordered_indices.into_iter().sum::<usize>().to_string();

    // Part 2
    let divider1: Element = serde_json::from_str("[[2]]").unwrap();
    let divider2_chars: Vec<char> = "[[6]]".chars().collect();
    let divider2 = Element::from_char_array(&divider2_chars);

    let mut ordered_elems: Vec<Element> =
        pairings.into_iter().flat_map(|p| vec![p.0, p.1]).collect();

    ordered_elems.push(divider1.clone());
    ordered_elems.push(divider2.clone());

    ordered_elems.sort_by(|a, b| match a.is_ordered(b) {
        Cmp::Ordered => std::cmp::Ordering::Less,
        Cmp::Indetermined => std::cmp::Ordering::Equal,
        Cmp::NotOrdered => std::cmp::Ordering::Greater,
    });

    let (i1, _) = ordered_elems
        .iter()
        .enumerate()
        .find(|(_, e)| *e == &divider1)
        .unwrap();
    let (i2, _) = ordered_elems
        .iter()
        .enumerate()
        .find(|(_, e)| *e == &divider2)
        .unwrap();

    let part2 = ((i1 + 1) * (i2 + 1)).to_string();

    (part1, part2)
}

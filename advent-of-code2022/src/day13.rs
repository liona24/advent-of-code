#[derive(Debug, Clone)]
enum MyArray {
    Number(u32),
    Vec(Vec<MyArray>),
}

fn parse_array(line: &str) -> MyArray {
    let mut inner = vec![Vec::new()];

    let mut char_accum = Vec::new();
    for char in line.chars() {
        match char {
            '0'..='9' => {
                char_accum.push(char);
            }
            '[' => {
                inner.push(Vec::new());
            }
            ']' => {
                if !char_accum.is_empty() {
                    let s = String::from_iter(char_accum.drain(..));
                    inner
                        .last_mut()
                        .expect("invalid input: missmatched ]")
                        .push(MyArray::Number(s.parse().expect("invalid input")))
                }
                let top = inner.pop().expect("invalid input: missmatched ]");
                inner.last_mut().unwrap().push(MyArray::Vec(top));
            }
            ',' => {
                if !char_accum.is_empty() {
                    let s = String::from_iter(char_accum.drain(..));
                    inner
                        .last_mut()
                        .expect("invalid input: missmatched ]")
                        .push(MyArray::Number(s.parse().expect("invalid input")))
                }
            }
            _ => {}
        }
    }

    assert!(char_accum.is_empty());
    assert!(inner.len() == 1);
    assert!(inner[0].len() == 1);

    inner.pop().unwrap().pop().unwrap()
}

impl PartialEq for MyArray {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MyArray::Number(left), MyArray::Number(right)) => left == right,
            (MyArray::Vec(left), MyArray::Vec(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for MyArray {}

impl PartialOrd for MyArray {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyArray {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (MyArray::Number(left), MyArray::Number(right)) => left.cmp(right),
            (MyArray::Number(left), right) => MyArray::Vec(vec![MyArray::Number(*left)]).cmp(right),
            (left, MyArray::Number(right)) => {
                left.cmp(&MyArray::Vec(vec![MyArray::Number(*right)]))
            }
            (MyArray::Vec(left), MyArray::Vec(right)) => {
                for (left_i, right_i) in left.into_iter().zip(right) {
                    let result = left_i.cmp(right_i);

                    if result != std::cmp::Ordering::Equal {
                        return result;
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

fn process_pair(pair: [Option<&str>; 2]) -> bool {
    let a1 = parse_array(pair[0].expect("invalid input"));
    let a2 = parse_array(pair[1].expect("invalid input"));

    a1.partial_cmp(&a2) == Some(std::cmp::Ordering::Less)
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut pair = [None; 2];
    let mut i = 0;

    let mut count = 0;
    let mut ipair = 0;

    for line in input {
        if line.is_empty() {
            i = 0;

            ipair += 1;
            if process_pair(pair) {
                count += ipair;
            }
        } else {
            pair[i] = Some(line);
            i += 1;
        }
    }

    if i != 0 {
        ipair += 1;
        if process_pair(pair) {
            count += ipair;
        }
    }

    count
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let d1 = MyArray::Vec(vec![MyArray::Vec(vec![MyArray::Number(2)])]);
    let d2 = MyArray::Vec(vec![MyArray::Vec(vec![MyArray::Number(6)])]);

    let mut packets = vec![d1.clone(), d2.clone()];

    for line in input {
        if !line.is_empty() {
            packets.push(parse_array(line))
        }
    }

    packets.sort();
    packets
        .into_iter()
        .enumerate()
        .filter(|(_i, el)| el == &d1 || el == &d2)
        .map(|(i, _el)| i + 1)
        .fold(1, |accum, i| accum * i)
}

impl_dayx!("13", solve1, solve2);

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    runs: [u8; 5],
}

fn byte_to_card(b: &u8) -> Option<u8> {
    match b {
        b'2'..=b'9' => Some(b - b'0'),
        b'T' => Some(10),
        b'J' => Some(11),
        b'Q' => Some(12),
        b'K' => Some(13),
        b'A' => Some(14),
        _ => None,
    }
}

impl Hand {
    fn new(cards: &[u8]) -> Option<Self> {
        let cards: [u8; 5] = cards
            .into_iter()
            .map(byte_to_card)
            .collect::<Option<Vec<u8>>>()?
            .try_into()
            .ok()?;
        let mut runs = [0; 5];

        let mut sorted = cards.clone();
        sorted.sort_unstable();

        let mut run = 0;
        let mut prev = sorted[0];
        for c in sorted.into_iter() {
            if c == prev {
                run += 1;
            } else {
                prev = c;
                runs[runs.len() - run] += 1;
                run = 1;
            }
        }
        runs[runs.len() - run] += 1;

        Some(Self { cards, runs })
    }

    fn new2(cards: &[u8]) -> Option<Self> {
        let mut cards: [u8; 5] = cards
            .into_iter()
            .map(byte_to_card)
            .collect::<Option<Vec<u8>>>()?
            .try_into()
            .ok()?;

        for card in cards.iter_mut() {
            if *card == 11 {
                *card = 0;
            }
        }

        let mut runs = [[0; 5]; 13];

        for (j, runs) in runs.iter_mut().enumerate() {
            let j = (j + 2) as u8;

            let mut sorted = cards.clone();
            for card in sorted.iter_mut() {
                if *card == 0 {
                    *card = j;
                }
            }
            sorted.sort_unstable();

            let mut run = 0;
            let mut prev = sorted[0];
            for c in sorted.into_iter() {
                if c == prev {
                    run += 1;
                } else {
                    prev = c;
                    runs[runs.len() - run] += 1;
                    run = 1;
                }
            }
            runs[runs.len() - run] += 1;
        }

        let runs = runs.into_iter().max().unwrap();
        Some(Self { cards, runs })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.runs.cmp(&other.runs) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut hands = Vec::new();
    for line in input {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = Hand::new(hand.as_bytes()).unwrap();
        let bid = u64::from_str_radix(bid, 10).unwrap();
        hands.push((hand, bid));
    }

    hands.sort();

    let mut sum = 0;
    for (i, (_, bid)) in hands.into_iter().enumerate() {
        let i = i as u64 + 1;
        sum += i * bid;
    }

    sum
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut hands = Vec::new();
    for line in input {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = Hand::new2(hand.as_bytes()).unwrap();
        let bid = u64::from_str_radix(bid, 10).unwrap();
        hands.push((hand, bid));
    }

    hands.sort();

    let mut sum = 0;
    for (i, (_, bid)) in hands.into_iter().enumerate() {
        let i = i as u64 + 1;
        sum += i * bid;
    }

    sum
}

impl_dayx!("07", solve1, solve2);

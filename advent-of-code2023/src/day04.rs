use std::collections::BTreeSet;

struct Card {
    id: usize,
    winners: BTreeSet<usize>,
    selection: BTreeSet<usize>,
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (head, tail) = value.split_once(": ").ok_or("':' missing")?;

        let (_, id) = head.split_once(' ').ok_or("' ' missing")?;
        let id = usize::from_str_radix(id.trim(), 10).map_err(|e| e.to_string())?;

        let (winners_str, selection_str) = tail.split_once(" | ").ok_or("'|' missing")?;
        let winners: BTreeSet<_> = winners_str
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|num| usize::from_str_radix(num, 10).map_err(|e| e.to_string()))
            .collect::<Result<BTreeSet<usize>, Self::Error>>()?;
        let selection: BTreeSet<_> = selection_str
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|num| usize::from_str_radix(num, 10).map_err(|e| e.to_string()))
            .collect::<Result<BTreeSet<usize>, Self::Error>>()?;

        Ok(Card {
            id,
            winners,
            selection,
        })
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut sum = 0;

    for line in input {
        let card: Card = line.as_str().try_into().unwrap();

        let winners = card.selection.intersection(&card.winners).count();
        if winners > 0 {
            sum += 1u64 << (winners - 1);
        }
    }

    sum
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut card_counts: Vec<usize> = vec![1; input.len()];

    for (i, line) in input.iter().enumerate() {
        let card: Card = line.as_str().try_into().unwrap();
        assert_eq!(card.id, i + 1);

        let winners = card.selection.intersection(&card.winners).count();
        for j in 0..winners {
            assert!(i + 1 + j < card_counts.len());
            card_counts[i + 1 + j] += card_counts[i];
        }
    }

    card_counts.into_iter().sum::<usize>()
}

impl_dayx!("04", solve1, solve2);

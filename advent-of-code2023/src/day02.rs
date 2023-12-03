struct Draw {
    r: usize,
    g: usize,
    b: usize,
}

struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl TryFrom<&str> for Game {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (head, tail) = value.split_once(": ").ok_or("':' missing")?;

        let (_, id) = head.split_once(' ').ok_or("' ' missing")?;
        let id = usize::from_str_radix(id, 10).map_err(|e| e.to_string())?;

        let mut draws = Vec::new();
        for draw_str in tail.split("; ") {
            let mut draw = Draw { r: 0, g: 0, b: 0 };
            for part in draw_str.split(", ") {
                let (num, name) = part.split_once(' ').ok_or("' ' missing")?;

                let num = usize::from_str_radix(num, 10).map_err(|e| e.to_string())?;

                match name {
                    "red" => draw.r += num,
                    "green" => draw.g += num,
                    "blue" => draw.b += num,
                    _ => return Err("invalid name".to_string()),
                }
            }

            draws.push(draw)
        }

        Ok(Game { id, draws })
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut sum = 0;
    for line in input {
        let game: Game = line.as_str().try_into().unwrap();

        if game
            .draws
            .iter()
            .all(|draw| draw.r <= 12 && draw.g <= 13 && draw.b <= 14)
        {
            sum += game.id;
        }
    }

    sum
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut sum = 0;
    for line in input {
        let game: Game = line.as_str().try_into().unwrap();

        let r = game.draws.iter().map(|draw| draw.r).max().unwrap();
        let g = game.draws.iter().map(|draw| draw.g).max().unwrap();
        let b = game.draws.iter().map(|draw| draw.b).max().unwrap();

        sum += r * g * b;
    }

    sum
}

impl_dayx!("02", solve1, solve2);

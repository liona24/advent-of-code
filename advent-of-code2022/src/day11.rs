use std::cell::RefCell;

struct Monkey<'op> {
    items: Vec<u64>,
    test: u64,
    target_true: usize,
    target_false: usize,
    op: &'op dyn Fn(u64) -> u64,

    inspect_counter: u64,
}

impl<'op> Monkey<'op> {
    fn new(
        items: Vec<u64>,
        test: u64,
        target_true: usize,
        target_false: usize,
        op: &'op dyn Fn(u64) -> u64,
    ) -> Self {
        Self {
            items,
            test,
            target_true,
            target_false,
            op,
            inspect_counter: 0,
        }
    }

    fn inspect(&mut self, monkeys: &[RefCell<Self>]) {
        for item in self.items.drain(..) {
            self.inspect_counter += 1;

            let item = (self.op)(item) / 3;

            if item % self.test == 0 {
                monkeys[self.target_true].borrow_mut().items.push(item);
            } else {
                monkeys[self.target_false].borrow_mut().items.push(item);
            }
        }
    }

    fn inspect2(&mut self, monkeys: &[RefCell<Self>], modulus: u64) {
        for item in self.items.drain(..) {
            self.inspect_counter += 1;

            let item = (self.op)(item) % modulus;

            if item % self.test == 0 {
                monkeys[self.target_true].borrow_mut().items.push(item);
            } else {
                monkeys[self.target_false].borrow_mut().items.push(item);
            }
        }
    }
}

fn get_monkeys() -> Vec<RefCell<Monkey<'static>>> {
    // vec![
    //     RefCell::new(Monkey::new(vec![79, 68], 23, 2, 3, &|x| x * 19)),
    //     RefCell::new(Monkey::new(vec![54, 65, 75, 74], 19, 2, 0, &|x| x + 6)),
    //     RefCell::new(Monkey::new(vec![79, 60, 97], 13, 1, 3, &|x| x * x)),
    //     RefCell::new(Monkey::new(vec![74], 17, 0, 1, &|x| x + 3)),
    // ]

    vec![
        RefCell::new(Monkey::new(vec![75, 63], 11, 7, 2, &|x| x * 3)),
        RefCell::new(Monkey::new(
            vec![65, 79, 98, 77, 56, 54, 83, 94],
            2,
            2,
            0,
            &|x| x + 3,
        )),
        RefCell::new(Monkey::new(vec![66], 5, 7, 5, &|x| x + 5)),
        RefCell::new(Monkey::new(vec![51, 89, 90], 7, 6, 4, &|x| x * 19)),
        RefCell::new(Monkey::new(
            vec![75, 94, 66, 90, 77, 82, 61],
            17,
            6,
            1,
            &|x| x + 1,
        )),
        RefCell::new(Monkey::new(vec![53, 76, 59, 92, 95], 19, 4, 3, &|x| x + 2)),
        RefCell::new(Monkey::new(vec![81, 61, 75, 89, 70, 92], 3, 0, 1, &|x| {
            x * x
        })),
        RefCell::new(Monkey::new(vec![81, 86, 62, 87], 13, 3, 5, &|x| x + 8)),
    ]
}

fn solve1(_input: &[String]) -> impl std::fmt::Display {
    let monkeys = get_monkeys();

    for _ in 0..20 {
        for monkey in monkeys.iter() {
            monkey.borrow_mut().inspect(&monkeys);
        }
    }

    let mut counters: Vec<u64> = monkeys
        .into_iter()
        .map(|m| m.borrow().inspect_counter)
        .collect();

    counters.sort();
    counters[counters.len() - 1] * counters[counters.len() - 2]
}

fn solve2(_input: &[String]) -> impl std::fmt::Display {
    let monkeys = get_monkeys();

    // assuming tests are primes
    let modulus = monkeys
        .iter()
        .map(|m| m.borrow().test)
        .reduce(|prod, x| prod * x)
        .expect("at least one monkey required");

    for _ in 0..10000 {
        for monkey in monkeys.iter() {
            monkey.borrow_mut().inspect2(&monkeys, modulus);
        }
    }

    let mut counters: Vec<u64> = monkeys
        .into_iter()
        .map(|m| m.borrow().inspect_counter)
        .collect();

    counters.sort();

    counters[counters.len() - 1] * counters[counters.len() - 2]
}

impl_dayx!("11", solve1, solve2);

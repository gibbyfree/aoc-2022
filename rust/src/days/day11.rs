use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
/// part 1 of this was actually a lot of fun, but i don't like the number theory problems
/// learned LCM trick from reddit, all other attempts failed
/// also wasted a lot of time before fixing my solution with find+replace i32 -> i64 :')

pub fn solve() -> SolutionPair {
    let raw_input = include_str!("/home/gib/aoc/input/11.in");
    let mut div_vals: Vec<i64> = Vec::new();
    let monkey_tuples = raw_input
        .split("\n\n")
        .enumerate()
        .map(|(i, chunk)| {
            let mut mb = MonkeyBuilder::default();
            for line in chunk.split("\n").skip(1) {
                let trim = line.trim();
                if trim.starts_with("Start") {
                    let item_options: Vec<Option<i64>> = trim
                        .split(" ")
                        .map(|x| {
                            let mut x_parse = x;
                            if x.contains(',') {
                                x_parse = &x[0..x.len() - 1];
                            }
                            let try_num = x_parse.parse::<i64>();
                            match try_num {
                                Ok(num) => return Some(num),
                                _ => return None,
                            }
                        })
                        .collect();
                    mb.set_items(
                        item_options
                            .into_iter()
                            .filter_map(|x| x)
                            .collect::<Vec<i64>>(),
                    )
                } else if trim.starts_with("Op") {
                    let mut parts = trim.split(" ").collect::<Vec<&str>>();
                    let quant = parts.pop().unwrap();
                    if quant == "old" {
                        mb.set_op_quant(0); // arbitrary 'flag' for this outlier quant
                    } else {
                        mb.set_op_quant(quant.parse::<i64>().unwrap());
                    }
                    mb.set_operator(parts.pop().unwrap());
                } else if trim.starts_with("Test") {
                    let val = trim.split(" ").last().unwrap().parse::<i64>().unwrap();
                    mb.set_div(val);
                    div_vals.push(val);
                } else if trim.starts_with("If true") {
                    mb.set_true_monkey(trim.split(" ").last().unwrap().parse::<usize>().unwrap());
                } else {
                    // must be the false monkey
                    mb.set_false_monkey(trim.split(" ").last().unwrap().parse::<usize>().unwrap());
                }
            }
            return (i, mb.build());
        })
        .collect::<Vec<(usize, Monkey)>>();

    let monkey_map: HashMap<usize, Monkey> = HashMap::from_iter(monkey_tuples);
    let second_map = monkey_map.clone();
    let lcm: i64 = div_vals.iter().product();
    (
        Solution::Str(simulate_simians(monkey_map, 20, true, lcm).to_string()),
        Solution::Str(simulate_simians(second_map, 10000, false, lcm).to_string()),
    )
}

pub fn simulate_simians(
    mut monkeys: HashMap<usize, Monkey>,
    rounds: i64,
    worry: bool,
    lcm: i64,
) -> usize {
    let mut round_counter = 0;
    let mut inspection_counts: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 8 monkeys
    while round_counter < rounds {
        for n in 0..=7 {
            let current: &mut Monkey = monkeys.get_mut(&n).unwrap();
            let mut vals_to_throw: Vec<(usize, i64)> = Vec::new();

            let mut inspections = 0;
            for (_i, item) in current.items.iter().enumerate() {
                inspections += 1;
                let inc_worry: i64 = if current.op_quant == 0 {
                    (item * item) as i64
                } else if current.operator == Operator::Add {
                    (item + current.op_quant) as i64
                } else {
                    (item * current.op_quant) as i64
                };


                let dec_worry: i64 = if worry {
                    inc_worry / 3
                } else {
                    inc_worry % lcm
                };

                // throw it
                let target = if dec_worry % current.div == 0 {
                    current.true_monkey
                } else {
                    current.false_monkey
                };
                vals_to_throw.push((target, dec_worry));
            }
            inspection_counts[n] = inspection_counts[n] + inspections;
            current.items.clear(); // monkey throws all

            for val in vals_to_throw {
                let thrown = monkeys.get_mut(&val.0).unwrap();
                thrown.items.push(val.1);
            }
        }
        round_counter += 1;
    }
    inspection_counts.sort_unstable();
    inspection_counts.pop().unwrap() * inspection_counts.pop().unwrap()
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Operator {
    Mult,
    Add,
}

// items: starting items
// operator: + or *
// op_quant: old + x, old * x
// div: divisible by
// true/false_monkey: table id of monkey to throw to
#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<i64>,
    operator: Operator,
    op_quant: i64,
    div: i64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    pub fn new(
        items: Vec<i64>,
        operator: Operator,
        op_quant: i64,
        div: i64,
        true_monkey: usize,
        false_monkey: usize,
    ) -> Self {
        Self {
            items,
            operator,
            op_quant,
            div,
            true_monkey,
            false_monkey,
        }
    }
}

// shout-out to all my Object-Oriented Design warriors
pub trait Builder {
    type OutputType;
    fn set_items(&mut self, items: Vec<i64>);
    fn set_operator(&mut self, operator: &str);
    fn set_op_quant(&mut self, op_quant: i64);
    fn set_div(&mut self, div: i64);
    fn set_true_monkey(&mut self, true_monkey: usize);
    fn set_false_monkey(&mut self, false_monkey: usize);
    fn build(self) -> Self::OutputType;
}

#[derive(Default)]
pub struct MonkeyBuilder {
    items: Option<Vec<i64>>,
    operator: Option<Operator>,
    op_quant: Option<i64>,
    div: Option<i64>,
    true_monkey: Option<usize>,
    false_monkey: Option<usize>,
}

impl Builder for MonkeyBuilder {
    type OutputType = Monkey;

    fn set_items(&mut self, items: Vec<i64>) {
        self.items = Some(items);
    }

    fn set_operator(&mut self, operator: &str) {
        match operator {
            "*" => self.operator = Some(Operator::Mult),
            "+" => self.operator = Some(Operator::Add),
            _ => panic!("unexpected operator"),
        }
    }

    fn set_op_quant(&mut self, op_quant: i64) {
        self.op_quant = Some(op_quant);
    }

    fn set_div(&mut self, div: i64) {
        self.div = Some(div);
    }

    fn set_true_monkey(&mut self, true_monkey: usize) {
        self.true_monkey = Some(true_monkey);
    }

    fn set_false_monkey(&mut self, false_monkey: usize) {
        self.false_monkey = Some(false_monkey);
    }

    fn build(self) -> Self::OutputType {
        Monkey::new(
            self.items.expect("items missing"),
            self.operator.expect("operator missing"),
            self.op_quant.expect("op quant missing"),
            self.div.expect("divisible missing"),
            self.true_monkey.expect("true monkey missing"),
            self.false_monkey.expect("false monkey missing"),
        )
    }
}

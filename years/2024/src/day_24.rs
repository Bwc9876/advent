use std::collections::{HashMap, HashSet, VecDeque};

use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day24;

pub type Wires = HashMap<String, bool>;
pub type Gates = HashMap<(String, String, String), Gate>;

fn find_gate<'a>(gates: &'a Gates, lhs: &str, rhs: &str, op: Op) -> Option<&'a Gate> {
    gates.get(&(lhs.to_string(), rhs.to_string(), format!("{op:?}")))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Self::And => lhs & rhs,
            Self::Or => lhs | rhs,
            Self::Xor => lhs ^ rhs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gate {
    lhs: String,
    op: Op,
    rhs: String,
    target: String,
}

impl Gate {
    pub fn parse(raw: &str) -> Self {
        let mut s = raw.split(" ");
        let lhs = s.next().unwrap().to_string();
        let op = s.next().unwrap();
        let rhs = s.next().unwrap().to_string();
        let target = s.skip(1).next().unwrap().to_string();
        let op = match op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!(),
        };
        Self {
            lhs,
            rhs,
            target,
            op,
        }
    }

    pub fn run(&self, wires: &mut Wires) -> bool {
        if let (Some(&lhs), Some(&rhs)) = (wires.get(&self.lhs), wires.get(&self.rhs)) {
            wires.insert(self.target.clone(), self.op.eval(lhs, rhs));
            true
        } else {
            false
        }
    }
}

enum AdderTestResult {
    Okay(String),
    SwapNeeded(String, String),
    CompletelyWrong,
    End,
}

impl AdderTestResult {
    fn unwrap_okay(self) -> String {
        match self {
            Self::Okay(s) => s,
            _ => panic!(),
        }
    }
}

// The first adder is a half adder and needs to follow this format:
//
// x00, y00 -> AND -> [carry output wire]
// x00, y00 -> XOR -> z00
//
fn test_first_adder(gates: &Gates) -> AdderTestResult {
    let x0 = "x00".to_string();
    let y0 = "y00".to_string();
    if let Some(Gate { target, .. }) = find_gate(gates, &x0, &y0, Op::And) {
        if let Some(Gate {
            target: z_target, ..
        }) = find_gate(gates, &x0, &y0, Op::Xor)
        {
            if z_target == "z00" {
                if target == "z01" {
                    AdderTestResult::End
                } else {
                    AdderTestResult::Okay(target.to_string())
                }
            } else {
                AdderTestResult::SwapNeeded(target.to_string(), z_target.to_string())
            }
        } else {
            AdderTestResult::CompletelyWrong
        }
    } else {
        AdderTestResult::CompletelyWrong
    }
}

// Full adders must follow this pattern:
//
// 1. x[], y[] -> AND -> [xy_and_target]
// 2. x[], y[] -> XOR -> [xy_xor_target]
// 3. [carry_input], [xy_xor_target] -> XOR -> z[]
// 4. [carry_input], [xy_xor_target] -> AND -> [carry_xy_target]
// 5. [carry_xy_target], [xy_and_target] -> OR -> [carry_output]
//
// Of the gates, the ones that can swap outputs without creating a loop are as follows:
// A. #1 & #2
// B. #3 & #5
// C. #3 & #4
// D. #1 & #3
//
// These ones would result in insane or same output:
// #1 & #4, this actually results in the same output no matter what
// #2 & #3, this will result in #3's own output be its input which is invalid to this problem
// #2 & #4, ^
// #2 & #5, ^
// #4 & #5, ^
//
fn test_adder(num: usize, carry_input: &String, gates: &Gates) -> AdderTestResult {
    let x = format!("x{num:02}");
    let y = format!("y{num:02}");
    let z = format!("z{num:02}");
    if let (
        Some(Gate {
            target: xy_and_target,
            ..
        }),
        Some(Gate {
            target: xy_xor_target,
            ..
        }),
    ) = (
        find_gate(gates, &x, &y, Op::And),
        find_gate(gates, &x, &y, Op::Xor),
    ) {
        if let Some(Gate {
            target: z_target, ..
        }) = find_gate(gates, &carry_input, xy_xor_target.as_str(), Op::Xor)
        {
            if *z_target != z {
                // We know gate #3 is pointing to the wrong target, as it should be pointing to z[]
                // We can now confidently swap z[] and this target
                // This covers invalid state B, C, and D
                return AdderTestResult::SwapNeeded(z_target.to_string(), z);
            }
        } else {
            // We know that gate #2 has an invalid output, and since the only case that involves #2
            // is case A, we know that we're swapped with gate #1, we simply need to return the two
            // targets we already have
            return AdderTestResult::SwapNeeded(
                xy_and_target.to_string(),
                xy_xor_target.to_string(),
            );
        };

        // From here we've checked all test cases, we can confidently attempt to find the carry
        // output now
        let carry_xy_target = &find_gate(&gates, xy_xor_target, &carry_input, Op::And)
            .expect("Failed to find carry_xy_target")
            .target;
        let carry_out = &find_gate(&gates, carry_xy_target, xy_and_target, Op::Or)
            .expect("Failed to find carry_out")
            .target;
        if *carry_out == format!("z{:02}", num + 1) {
            AdderTestResult::End
        } else {
            AdderTestResult::Okay(carry_out.to_string())
        }
    } else {
        AdderTestResult::CompletelyWrong
    }
}

fn swap_outputs(gates: &mut Gates, out1: &String, out2: &String) {
    gates.values_mut().for_each(|g| {
        if g.target == *out1 {
            g.target = out2.to_string();
        } else if g.target == *out2 {
            g.target = out1.to_string();
        }
    });
}

// 0,1,
// z16,tdv,hnd,z09,z23,bks,nrn,tjp

impl Day for Day24 {
    day_stuff!(24, "4", "bks,hnd,nrn,tdv,tjp,z09,z16,z23", (Wires, Gates));

    fn part_1((mut wires, gates): Self::Input) -> Option<String> {
        let mut all_zs = gates
            .values()
            .filter(|g| g.target.starts_with('z'))
            .map(|g| &g.target)
            .collect::<Vec<_>>();
        all_zs.sort();
        all_zs.dedup();

        let mut current_zs = HashSet::<&String>::with_capacity(all_zs.len());

        let mut queue = gates.values().collect::<VecDeque<_>>();

        while let Some(gate) = queue.pop_front()
            && current_zs.len() < all_zs.len()
        {
            if gate.run(&mut wires) {
                if gate.target.starts_with('z') {
                    current_zs.insert(&gate.target);
                }
            } else {
                queue.push_back(gate);
            }
        }

        let ans = all_zs.into_iter().enumerate().fold(0_usize, |acc, (i, z)| {
            let wire = wires.get(z).unwrap();
            if *wire {
                acc | (1 << i)
            } else {
                acc
            }
        });

        Some(ans.to_string())
    }

    fn part_2((_, mut gates): Self::Input) -> Option<String> {
        let mut swapped = Vec::with_capacity(8);
        let mut current_carry = String::new();

        for i in 0.. {
            let res = if i == 0 {
                test_first_adder(&gates)
            } else {
                test_adder(i, &current_carry, &gates)
            };

            match res {
                AdderTestResult::Okay(new_carry) => {
                    current_carry = new_carry;
                }
                AdderTestResult::End => {
                    break;
                }
                AdderTestResult::CompletelyWrong => {
                    panic!("Wrong adder");
                }
                AdderTestResult::SwapNeeded(l, r) => {
                    swap_outputs(&mut gates, &l, &r);
                    current_carry = test_adder(i, &current_carry, &gates).unwrap_okay();
                    swapped.push(l);
                    swapped.push(r);
                }
            };
        }

        swapped.sort();

        Some(swapped.join(","))
    }

    fn parse_input(input: &str) -> Self::Input {
        let (inits, gates) = input.trim().split_once("\n\n").unwrap();

        let wires = inits
            .lines()
            .map(|l| {
                let (name, val) = l.split_once(": ").unwrap();
                (name.to_string(), val == "1")
            })
            .collect::<HashMap<_, _>>();

        let gates = gates
            .lines()
            .flat_map(|l| {
                let gate = Gate::parse(l);
                let op_str = format!("{:?}", gate.op);
                [
                    (
                        (gate.lhs.clone(), gate.rhs.clone(), op_str.clone()),
                        gate.clone(),
                    ),
                    ((gate.rhs.clone(), gate.lhs.clone(), op_str), gate),
                ]
            })
            .collect::<HashMap<_, _>>();

        (wires, gates)
    }
}

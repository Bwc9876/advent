use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day17;

#[derive(Clone, Copy, Debug)]
pub struct Registers {
    a: u128,
    b: u128,
    c: u128,
}

impl Registers {
    fn with_combo_op(&self, combo_op: &ComboOperand) -> u128 {
        match combo_op {
            ComboOperand::RegA => self.a,
            ComboOperand::RegB => self.b,
            ComboOperand::RegC => self.c,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ComboOperand {
    Literal(u128),
    RegA,
    RegB,
    RegC,
    Reserved,
}

impl ComboOperand {
    fn from_u128(v: u128) -> Self {
        match v {
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            7 => Self::Reserved,
            o => Self::Literal(o),
        }
    }

    fn get_val(&self, regs: &Registers) -> u128 {
        match self {
            Self::Literal(v) => *v,
            Self::Reserved => panic!(),
            reg => regs.with_combo_op(reg),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(u128),
    Bst(ComboOperand),
    Jnz(u128),
    Bxc(()),
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    fn from_slice(ins: &[u128]) -> Self {
        let op = ins[0];
        let opr = ins[1];

        match op {
            0 => Self::Adv(ComboOperand::from_u128(opr)),
            1 => Self::Bxl(opr),
            2 => Self::Bst(ComboOperand::from_u128(opr)),
            3 => Self::Jnz(opr),
            4 => Self::Bxc(()),
            5 => Self::Out(ComboOperand::from_u128(opr)),
            6 => Self::Bdv(ComboOperand::from_u128(opr)),
            7 => Self::Cdv(ComboOperand::from_u128(opr)),
            _ => unreachable!(),
        }
    }

    fn execute(&self, ip: u128, regs: &mut Registers) -> (u128, Option<u128>) {
        match self {
            Self::Adv(op) => {
                regs.a /= 2_u128.pow(op.get_val(regs) as u32);
            }
            Self::Bxl(v) => {
                regs.b ^= v;
            }
            Self::Bst(op) => {
                regs.b = op.get_val(regs) % 8;
            }
            Self::Jnz(v) => {
                if regs.a != 0 {
                    return (*v, None);
                }
            }
            Self::Bxc(_) => {
                regs.b ^= regs.c;
            }
            Self::Out(op) => {
                return (ip + 2, Some(op.get_val(regs) % 8));
            }
            Self::Bdv(op) => {
                regs.b = regs.a / 2_u128.pow(op.get_val(regs) as u32);
            }
            Self::Cdv(op) => {
                regs.c = regs.a / 2_u128.pow(op.get_val(regs) as u32);
            }
        }

        (ip + 2, None)
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    regs: Registers,
    instructions: Vec<u128>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let (raw_regs, raw_program) = input.trim().split_once("\n\n").unwrap();
        let mut regs = raw_regs
            .lines()
            .map(|l| l.split_once(": ").unwrap().1.parse::<u128>().unwrap());
        let regs = Registers {
            a: regs.next().unwrap(),
            b: regs.next().unwrap(),
            c: regs.next().unwrap(),
        };
        let instructions = raw_program
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|s| s.parse::<u128>().unwrap())
            .collect::<Vec<_>>();

        Self { regs, instructions }
    }
}

impl Day for Day17 {
    day_stuff!(17, "4,6,3,5,6,3,5,2,1,0", "117440", Computer);

    fn part_1(mut input: Self::Input) -> Option<String> {
        let mut ip = 0;
        let mut out = Vec::with_capacity(20);
        while ip < input.instructions.len() - 1 {
            let (next_ip, output) = Instruction::from_slice(&input.instructions[ip..=ip + 1])
                .execute(ip as u128, &mut input.regs);
            if let Some(v) = output {
                out.push(v.to_string());
            }
            ip = next_ip as usize;
        }
        Some(out.join(","))
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let mut possible_a = Vec::with_capacity(1000);
        possible_a.push(0);
        for ins in input.instructions.iter().rev().copied() {
            possible_a = possible_a
                .into_iter()
                .flat_map(|a| {
                    let mut branch_possible = vec![];
                    for possible_bits in 0_u128..=7 {
                        let new_a = (a << 3) + possible_bits;
                        let mut ip = 0;
                        let mut regs = Registers {
                            a: new_a,
                            b: 0,
                            c: 0,
                        };
                        let val = loop {
                            if ip >= input.instructions.len() - 1 {
                                break None;
                            }
                            let (next_ip, output) =
                                Instruction::from_slice(&input.instructions[ip..=ip + 1])
                                    .execute(ip as u128, &mut regs);
                            if let Some(v) = output {
                                break Some(v);
                            }
                            ip = next_ip as usize;
                        };

                        if let Some(val) = val
                            && val == ins
                        {
                            branch_possible.push(new_a);
                        }
                    }
                    branch_possible
                })
                .collect();
        }
        let ans = possible_a.into_iter().min().map(|v| v.to_string()).unwrap();
        Some(ans)
    }

    fn parse_input(input: &str) -> Self::Input {
        Computer::parse(input)
    }
}

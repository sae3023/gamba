use std::io;
use std::io::Write;
use rand::prelude::SliceRandom;
use rand::RngCore;
use rand::rngs::ThreadRng;

const FORMAT_SYMBOL: &str = "X";
const SYMBOLS: [&str; 4] = ["\x1b[0;31m7\x1b[0m ", "🍒", "🍋", "🍇"];
const SLOT_MACHINE_FMT: &str = include_str!("./gamba.txt");
const SLOT_MACHINE_FMT_LENGTH: usize = SLOT_MACHINE_FMT.len();

struct SlotMachine {
    first: usize,
    second: usize,
    third: usize,
}

impl SlotMachine {

    pub fn initialize_at_random(rng: &mut ThreadRng) -> SlotMachine {
        let first: usize = (rng.next_u32() % SYMBOLS.len()) as usize;
        let second: usize = (rng.next_u32() % SYMBOLS.len()) as usize;
        let third: usize = (rng.next_u32() % SYMBOLS.len()) as usize;
        SlotMachine {
            first,
            second,
            third
        }
    }

    pub fn draw(&self) -> String {
        let mut result = Vec::with_capacity(SLOT_MACHINE_FMT_LENGTH + 5);
        let indexes = self.as_array();
        let mut count = 0;
        for byte in SLOT_MACHINE_FMT.as_bytes() {
            if *byte == b'X' {
                let symbol = SYMBOLS[indexes[count]].as_bytes();
                result.extend_from_slice(symbol);
            } else {
                result.push(*byte);
            }
        }
        String::from_utf8(result).unwrap()
    }

    pub fn proceed_state(&mut self) {
        self.first = SlotMachine::inc_wrapping(self.first);
        self.second = SlotMachine::inc_wrapping(self.second);
        self.third = SlotMachine::inc_wrapping(self.third);
    }

    fn as_array(&self) -> [usize; 3] {
        [*&self.first, *&self.second, &self.third]
    }

    fn inc_wrapping(x: usize) -> usize {
        (x + 1) % SYMBOLS.len()
    }
}

pub fn spin_once() {
    let mut random = rand::thread_rng();
    draw_machine(&mut random);
    print!("\x1b[2J");
    io::stdout().flush().unwrap();
    draw_machine(&mut random);
}

fn draw_machine(rng: &mut ThreadRng) {
    let mut buff = Vec::with_capacity(SLOT_MACHINE_FMT_LENGTH + 5);
    for byte in SLOT_MACHINE_FMT.as_bytes() {
        if *byte == b'X' {
            let symbol = SYMBOLS.choose(rng).unwrap().as_bytes();
            buff.extend_from_slice(symbol);
        } else {
            buff.push(*byte);
        }
    }
    let result = String::from_utf8(buff).unwrap();
    print!("{result}");
    io::stdout().flush().unwrap();
}
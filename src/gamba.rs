use std::io::{stdout, Write};
use rand::RngCore;
use rand::rngs::ThreadRng;

const FORMAT_SYMBOL: &str = "X";
const RED_SEVEN: &str = "\x1b[0;31m7\x1b[0m ";
const CHERRY: &str = "🍒";
const LEMON: &str = "🍋";
const GRAPE: &str = "🍇";
const SYMBOLS: [&str; 4] = [RED_SEVEN, CHERRY, LEMON, GRAPE];
const SLOT_MACHINE_FMT: &str = include_str!("./gamba.txt");
const SLOT_MACHINE_FMT_LENGTH: usize = SLOT_MACHINE_FMT.len();
const BUFFER_SIZE: usize = SLOT_MACHINE_FMT_LENGTH + (3 * 14);

pub struct SlotMachine {
    first: usize,
    second: usize,
    third: usize,
    state: Vec<u8>,
}

impl SlotMachine {
    pub fn initialize_at_random(rng: &mut ThreadRng) -> SlotMachine {
        let first: usize = (rng.next_u32() % SYMBOLS.len() as u32) as usize;
        let second: usize = (rng.next_u32() % SYMBOLS.len() as u32) as usize;
        let third: usize = (rng.next_u32() % SYMBOLS.len() as u32) as usize;
        let state = SlotMachine::make_buffer(first, second, third);
        SlotMachine {
            first,
            second,
            third,
            state,
        }
    }

    pub fn proceed_state(&mut self, n: usize) {
        if n >= 3 {
            self.first = SlotMachine::inc_wrapping(self.first);
        }
        if n >= 2 {
            self.second = SlotMachine::inc_wrapping(self.second);
        }
        if n >= 1 {
            self.third = SlotMachine::inc_wrapping(self.third);
        }
        // let mut changed_count: usize = 0;
        // let commands_to_emit = Vec::with_capacity(10);
        let mut offset = 0;
        for x in self.state.as_slice()[119..119 + 65 + 9 * 3].windows(4) {
            if SlotMachine::contains_emoji(x) {
                let place_cursor = format!("\x1B[4;{}f", offset + 5);
                let place_cursor = place_cursor.as_bytes();
                let toggle_insert_mode = "\x1B[2~".as_bytes();
                let emoji = SYMBOLS[self.first].as_bytes();
                let to_write = [place_cursor, toggle_insert_mode, emoji, toggle_insert_mode].concat();
                stdout().write(to_write.as_slice()).unwrap();
                stdout().flush().unwrap();
                SlotMachine::magic();
                break;
            } else if x.starts_with("7".as_bytes()) {
                // println!("amg");
            }
            offset += 1;
        }
    }

    pub fn draw_to_stdout(&self) {
        stdout().write(self.state.as_slice()).unwrap();
        stdout().flush().unwrap();
    }

    fn make_buffer(first: usize, second: usize, third: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(BUFFER_SIZE);
        let indexes = [first, second, third];
        let mut count = 0;
        for byte in SLOT_MACHINE_FMT.as_bytes() {
            if *byte == b'X' {
                let symbol = SYMBOLS[indexes[count]].as_bytes();
                result.extend_from_slice(symbol);
                count += 1;
            } else {
                result.push(*byte);
            }
        }
        result
    }

    fn contains_emoji(us: &[u8]) -> bool {
        us.starts_with(CHERRY.as_bytes()) || us.starts_with(GRAPE.as_bytes()) || us.starts_with(LEMON.as_bytes())
    }

    fn as_array(&self) -> [usize; 3] {
        [*&self.first, *&self.second, *&self.third]
    }

    fn inc_wrapping(x: usize) -> usize {
        (x + 1) % SYMBOLS.len()
    }

    fn magic() {
        println!();
        println!();
        println!();
        println!();
    }
}
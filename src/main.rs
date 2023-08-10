mod gamba;

use gamba::SlotMachine;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    let mut machine = SlotMachine::initialize_at_random(&mut rng);
    machine.draw_to_stdout();
    thread::sleep(Duration::from_secs(1));
    machine.proceed_state(3);
}

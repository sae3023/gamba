use std::thread;
use std::time::Duration;
use crate::gamba::SlotMachine;

pub fn one_two_three_spin() {
    let mut rng = rand::thread_rng();
    let mut machine = SlotMachine::initialize_at_random(&mut rng);
    let sleep_duration = Duration::from_millis(150);
    machine.draw_to_stdout();
    let mut x = 3;
    let mut y = 10;
    while x > 0 {
        while y > 0 {
            machine.proceed_state(x);
            thread::sleep(sleep_duration);
            y -= 1;
        }
        y = 10;
        x -= 1;
    }
}

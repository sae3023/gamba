mod gamba;

use gamba::SlotMachine;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    let mut machine = SlotMachine::initialize_at_random(&mut rng);
    machine.draw_to_stdout();
    thread::sleep(Duration::from_secs(1));
    machine.proceed_state(3);
    // fake_main();
}

fn fake_main() {
    println!("A BUNCH OF LEMONS THAT IM TOO LAZY TO PASTE");
    println!("A BUNCH OF GRAPES THAT IM TOO LAZY TO PASTE");
    println!("A BUNCH OF CHERRYS THAT IM TOO LAZY TO PASTE");
    thread::sleep(Duration::from_secs(1));
    print!("\x1B[2;4f");
    print!("🍇");


    println!();
    println!();
    println!();
}
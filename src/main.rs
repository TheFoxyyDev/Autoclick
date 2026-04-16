use std::{
    io::{self, BufRead},
    sync::mpsc,
    thread,
    time,
};

use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, MouseButton, MouseControllable};

fn main() {
    println!("Autoclicker started");
    println!("F8 = toggle, q = quit, h = help, s = change speed (default: 50)");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let cmd = line.unwrap().trim().to_string();
            tx.send(cmd).unwrap();
        }
    });

    let device_state = DeviceState::new();
    let mut enigo = Enigo::new();
    let mut clicking = false;
    let mut speed_delay = 50;
    let mut running = true;
    let mut waiting_for_speed = false;

    while running {
        if let Ok(cmd) = rx.try_recv() {
            if waiting_for_speed {
                match cmd.parse::<u64>() {
                    Ok(new_speed) => {
                        speed_delay = new_speed;
                        println!("Speed updated to {} ms", speed_delay);
                        if speed_delay <= 25 {
                            println!("Please be aware that speeds below 25 will have a higher risk of freezing your Computer")
                        }
                        waiting_for_speed = false;
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a number (ms):");
                    }
                }
            } else {
                match cmd.as_str() {
                    "q" => {
                        println!("Exiting...");
                        break;
                    }
                    "h" => {
                        println!("Commands:");
                        println!("  q - quit");
                        println!("  h - help");
                        println!("  s - set speed");
                        println!("  F8 - toggle autoclicker");
                    }
                    "s" => {
                        println!("Enter new speed (delay in ms between clicks):");
                        waiting_for_speed = true;
                    }
                    _ => println!("Unknown command. Type 'h' for help."),
                }
            }
        }

        let keys = device_state.get_keys();

        if keys.contains(&Keycode::F8) {
            clicking = !clicking;
            println!("Autoclicker {}", if clicking { "ON" } else { "OFF" });

            thread::sleep(time::Duration::from_millis(300));
        }

        if clicking {
            enigo.mouse_click(MouseButton::Left);
        }

        thread::sleep(time::Duration::from_millis(speed_delay));
    }
}
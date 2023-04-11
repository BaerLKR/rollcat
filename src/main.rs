// Licensed under the EUPL

// Copyright (c) 2023 Lovis Rentsch

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use colored::*;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let q = &args[1];
        match q.as_str() {
            "-f" => {
                normal();
            },
            _ => help(),
        };
    } else {
        normal();
    }
}

fn normal() {
    let stdin = io::stdin();
    let mut counter = 0;
    for l in stdin.lock().lines() {
        counter = counter + 1;
        let line = String::from(l.unwrap());
        rollcat(line, counter);
    }
}

fn rollcat(input: String, line: i32) { 
    let mut ccount = 0;
    let mut frq = 5;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let q = &args[1];
        match q.as_str() {
            "-f" => {
                if args.len() > 2 {
                    match args[2].to_owned().as_str() {
                        "1" => frq = 1,
                        "2" => frq = 5,
                        _ => println!("Error"),
                        
                    }
                }
            }
            _ => {}
        };
    }
    for c in input.chars() {
        ccount = ccount + 1;
        let color = color(&ccount, &line, frq);
        print!("{}", c.to_string().truecolor(color[0], color[1], color[2]));
    }
    print!("\n");
}

fn color(num: &i32, line: &i32, frq: i32) -> Vec<u8>{
    let mut color: Vec<u8> = vec![255, 10, 100];
    let mut mode: Vec<&str> = vec![".", ".", "."];
    let factor: u8 = frq as u8;
    let line = line.to_owned();
    let stelle = num.to_owned();
    for _c in 0..stelle + line {
        for i in 0..color.len() {
            if color[i] == 0 {
                mode[i] = "+";
            }

            if color[i] == 255 {
                mode[i] = "-";
            }
        }

        for i in 0..mode.len() {
            if mode[i] == "+" {
                color[i] += factor;
            } else {
                color[i] -= factor;
            }
        }
    }
    color
}

fn help() {
    let info = String::from("This is a simple rewrite of the LOLCAT command in rust");
    rollcat(info, 1);
    rollcat(String::from("  It will read standart input and make it pwetty owo"), 1);
    println!("");
    rollcat(String::from("  -h / --help / * => this help"), 1);
    rollcat(String::from("  -f [1 or 2] => the frequency (1 is slower and 2 faster)"), 1);
    println!("");
    rollcat(String::from("      by Lovis in Rust for fun"), 1);
    rollcat(String::from("      Licensed under EUPL 1.2"), 1);
}

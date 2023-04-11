// Licensed under the EUPL

// Copyright (c) 2023 Lovis Rentsch

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use colored::*;
use std::io::{self, BufRead, Read};

fn main() {
    let stdin = io::stdin();
    let mut counter = 0;
    for l in stdin.lock().lines() {
        counter = counter + 1;
        let line = String::from(l.unwrap());
        rollcat(line, counter);
    }
}

fn rollcat(input: String, line: i32) { 
    let color = color(&line);
    for c in input.chars() {
        // print!("{}", c.to_string().truecolor(color[0], color[1], color[2]));
        println!("{:?}", color);
    }
    print!("\n");
}

fn color(num: &i32) -> Vec<u8>{
    let mut color: Vec<u8> = vec![255, 105, 50];
    let mut mode: Vec<&str> = vec![".", ".", "."];
    let factor: u8 = 5;

    let anz = num.to_owned() as u8;

    for _c in 0..anz {
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

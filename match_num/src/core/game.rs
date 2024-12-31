// Created on Wed Jan 01 2025
// © 2025 BLACKHAND Studio. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io;
use std::io::Write;

fn get_int() -> u8 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number!");
            get_int()
        }
    }
}

pub fn run(rand_num: u8, count: &mut u8) -> bool {
    if *count == 0 {
        return false;
    }

    print!("Now, you have {count} life point.\nGuess the number! (1~100) : ");

    io::stdout().flush().expect("Failed to flush stdout");

    let input_num: u8 = get_int();
    *count -= 1;

    if input_num > rand_num {
        println!("\nDOWN\n");
        true
    } else if input_num < rand_num {
        println!("\nUP\n");
        true
    } else {
        false
    }
}

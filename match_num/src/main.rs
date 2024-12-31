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
mod core;

const MAX_COUNT: u8 = 10;
const FIRST: u8 = 1;
const LAST: u8 = 100;

fn main() {
    let rand_num: u8 = core::random::get_int(FIRST, LAST);
    let mut count: u8 = MAX_COUNT;

    let mut is_running: bool = true;

    while is_running {
        is_running = core::game::run(rand_num, &mut count);
    }

    if count > 0 {
        println!("\nCongraturation! YOU WIN!");
    } else {
        println!("\nOh, No! YOU LOSE...");
    }
}

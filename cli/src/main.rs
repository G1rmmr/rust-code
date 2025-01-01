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

mod manager;

use manager::Manager;
use std::io;

fn main() {
    let mut manager = Manager::new();

    manager.add_job(String::from("Rust 공부"));
    manager.add_job(String::from("운동하기"));
    manager.add_job(String::from("코드 리뷰"));

    println!("[작업 추가 후]");
    manager.list();

    manager.upd_job(String::from("Rust 공부"), true);
    println!("\n[작업 상태 업데이트 후]");
    manager.list();

    manager.del_job(String::from("운동하기"));
    println!("\n[작업 삭제 후]");
    manager.list();

    println!("\n[없는 작업 삭제 시도]");
    manager.del_job(String::from("운동하기"));
    manager.list();
}

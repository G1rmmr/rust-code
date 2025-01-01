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

pub struct Job {
    pub name: String,
    pub done: bool,
}

impl Job {
    pub fn new(name: String) -> Self {
        Self { name, done: false }
    }
}

pub struct Manager {
    jobs: Vec<Job>,
}

impl Manager {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, name: String) {
        let job = Job::new(name);
        self.jobs.push(job);
    }

    pub fn del_job(&mut self, name: String) {
        let size = self.jobs.len();

        for i in 0..size {
            if self.jobs[i].name == name {
                self.jobs.remove(i);
                return;
            }
        }
    }

    pub fn upd_job(&mut self, name: String, done: bool) {
        let size = self.jobs.len();

        for i in 0..size {
            if self.jobs[i].name == name {
                self.jobs[i].done = done;
                return;
            }
        }
    }

    pub fn list(&self) {
        for (index, job) in self.jobs.iter().enumerate() {
            println!(
                "{}. {} [{}]",
                index + 1,
                job.name,
                if job.done { "완료" } else { "미완료" }
            );
        }
    }
}

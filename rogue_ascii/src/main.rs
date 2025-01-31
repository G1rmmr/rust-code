mod game;
mod map;
mod ui;

use game::*;
use map::Map;
use ui::*;

use cursive::event::Key;
use cursive::Cursive;
use cursive::CursiveExt;

use std::sync::{Arc, Mutex};

const MAP_SIZE: usize = 32;

const PLAYER_INIT_X: usize = MAP_SIZE / 2;
const PLAYER_INIT_Y: usize = MAP_SIZE / 2;

fn main() {
    let mut siv = Cursive::default();

    let player = Arc::new(Mutex::new(Entity::spawn_new_player(
        PLAYER_INIT_X,
        PLAYER_INIT_Y,
    )));
    let map = Arc::new(Map::new(MAP_SIZE));

    init(&mut siv, &player.lock().unwrap(), &map);

    let player_clone = Arc::clone(&player);
    let map_clone = Arc::clone(&map);

    siv.add_global_callback(Key::Left, move |s| {
        let mut player = player_clone.lock().unwrap();
        movement(&mut player, -1, 0, map_clone.size);
        update(s, &player, &map_clone);
    });

    let player_clone = Arc::clone(&player);
    let map_clone = Arc::clone(&map);

    siv.add_global_callback(Key::Right, move |s| {
        let mut player = player_clone.lock().unwrap();
        movement(&mut player, 1, 0, map_clone.size);
        update(s, &player, &map_clone);
    });

    let player_clone = Arc::clone(&player);
    let map_clone = Arc::clone(&map);

    siv.add_global_callback(Key::Up, move |s| {
        let mut player = player_clone.lock().unwrap();
        movement(&mut player, 0, 1, map_clone.size);
        update(s, &player, &map_clone);
    });

    let player_clone = Arc::clone(&player);
    let map_clone = Arc::clone(&map);

    siv.add_global_callback(Key::Down, move |s| {
        let mut player = player_clone.lock().unwrap();
        movement(&mut player, 0, -1, map_clone.size);
        update(s, &player, &map_clone);
    });

    siv.run();
}

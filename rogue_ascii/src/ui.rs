use crate::game::*;
use crate::map::Map;

use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

pub fn update(siv: &mut Cursive, player: &Entity, map: &Map) {
    let hp_text = format!(
        "HP: {}/{}",
        player.hp.as_ref().unwrap().now,
        player.hp.as_ref().unwrap().max
    );

    let mut render = String::new();

    for row in &map.tiles {
        for cell in row {
            match cell {
                EntityType::Player => render.push('@'),
                EntityType::Monster => render.push('!'),
                EntityType::Item => render.push('$'),
                EntityType::Wall => render.push('#'),
                EntityType::Floor => render.push('.'),
            }
        }
        render.push('\n');
    }

    siv.call_on_name("hp", |view: &mut TextView| {
        view.set_content(hp_text);
    });

    siv.call_on_name("map", |view: &mut TextView| {
        view.set_content(render);
    });
}

pub fn init(siv: &mut Cursive, player: &Entity, map: &Map) {
    let hp_text = format!(
        "HP: {}/{}",
        player.hp.as_ref().unwrap().now,
        player.hp.as_ref().unwrap().max
    );

    let mut render = String::new();

    for row in &map.tiles {
        for cell in row {
            match cell {
                EntityType::Player => render.push('@'),
                EntityType::Monster => render.push('!'),
                EntityType::Item => render.push('$'),
                EntityType::Wall => render.push('#'),
                EntityType::Floor => render.push('.'),
            }
        }
        render.push('\n');
    }

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new(hp_text).with_name("now"))
                .child(TextView::new(render).with_name("map")),
        )
        .title("Roguelike Game")
        .button("Quit", |s| s.quit()),
    );
}

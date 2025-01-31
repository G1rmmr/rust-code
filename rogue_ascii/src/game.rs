use rand::Rng;

// Entities
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Player,
    Monster,
    Item,
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Entity {
    pub id: EntityType,
    pub pos: Position,
    pub hp: Option<Health>,
    pub inven: Option<Inventory>,
    pub ai: Option<AI>,
}

impl Entity {
    pub fn spawn_new_player(x: usize, y: usize) -> Self {
        Self {
            id: EntityType::Player,
            pos: Position { x, y },
            hp: Some(Health { now: 100, max: 100 }),
            inven: Some(Inventory { items: vec![] }),
            ai: None,
        }
    }

    pub fn spawn_new_monster(x: usize, y: usize, behavior: BehaviorType) -> Self {
        Self {
            id: EntityType::Monster,
            pos: Position { x, y },
            hp: Some(Health { now: 30, max: 30 }),
            inven: None,
            ai: Some(AI { behavior }),
        }
    }
}

// Components
#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Health {
    pub now: i8,
    pub max: i8,
}

#[derive(Debug)]
pub struct Inventory {
    pub items: Vec<String>,
}

#[derive(Debug)]
pub enum BehaviorType {
    Passive,
    Aggressive,
}

#[derive(Debug)]
pub struct AI {
    pub behavior: BehaviorType,
}

// Systems
pub fn heal(entity: &mut Entity, amount: i8) {
    if let Some(hp) = &mut entity.hp {
        hp.now += amount;
        if hp.now > hp.max {
            hp.now = hp.max;
        }
        println!("HEAL: {} (HP: {}/{})", amount, hp.now, hp.max);
    }
}

pub fn pick_up_item(entity: &mut Entity, item_name: &str) {
    if let Some(inven) = &mut entity.inven {
        inven.items.push(item_name.to_string());
        println!("PICK UP: {}", item_name);
    }
}

pub fn print_status(entity: &Entity) {
    println!("Entity Type: {:?}", entity.id);
    if let Some(hp) = &entity.hp {
        println!("HP: {}/{}", hp.now, hp.max);
    }
    if let Some(inven) = &entity.inven {
        println!("Inventory: {:?}", inven.items);
    }
}

pub fn movement(entity: &mut Entity, dx: isize, dy: isize, size: usize) {
    let nx = (entity.pos.x as isize + dx) as usize;
    let ny = (entity.pos.y as isize + dy) as usize;

    if nx < size && ny < size {
        entity.pos.x = nx;
        entity.pos.y = ny;
    }
}

pub fn monster_ai(monsters: &mut Vec<Entity>, size: usize) {
    let mut rng = rand::rng();
    for monster in monsters.iter_mut() {
        if let Some(ai) = &monster.ai {
            match ai.behavior {
                BehaviorType::Passive => {
                    if rng.random_range(0..10) < 1 {
                        let dx: i8 = rng.random_range(-1..=1);
                        let dy: i8 = rng.random_range(-1..=1);
                        movement(monster, dx as isize, dy as isize, size);
                    }
                }
                BehaviorType::Aggressive => {
                    if rng.random_range(0..10) < 5 {
                        let dx: i8 = rng.random_range(-1..=1);
                        let dy: i8 = rng.random_range(-1..=1);
                        movement(monster, dx as isize, dy as isize, size);
                    }
                }
            }
        }
    }
}

pub fn combat(player: &mut Entity, monster: &mut Entity) {
    if let (Some(player_hp), Some(monster_hp)) = (&mut player.hp, &mut monster.hp) {
        println!("ATTACK: Player -> Monster");
        monster_hp.now -= 10;

        if monster_hp.now > 0 {
            println!("ATTACK: Monster -> Player");
            player_hp.now -= 5;
        } else {
            println!("DEATH : Monster");
        }
    }
}

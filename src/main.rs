#![allow(unused)]
use rand::{Rng, rngs::ThreadRng};

#[derive(Debug)]
struct Floor {
    enemys: Vec<Enemy>,
    loot: Loot,
    next_floor_loot: Vec<Loot>
}

impl Floor {
    fn new(enemys: Vec<Enemy>, loot: Loot, next_floor_loot: Vec<Loot>) -> Self {
        Self {
            enemys,
            loot,
            next_floor_loot
        }
    }

    fn generate_floor(level: &u32, loot: Loot, mut rng: &mut ThreadRng) -> Self {
        let num_enemys = rng.gen_range(1..=3);
        let level_compensation = match num_enemys {
            1 => 3,
            2 => 1,
            _ => 0
        };
        let mut enemys: Vec<Enemy> = vec![];
        let enemy_type_list = [EnemyType::DamageDealer, EnemyType::Debuffer, EnemyType::Spawner, EnemyType::Supporter, EnemyType::Tank];
        for _ in 0..num_enemys {
            let enemy_type = enemy_type_list[rng.gen_range(0..enemy_type_list.len()) as usize];
            enemys.push(Enemy::generate_enemy(&(*level + level_compensation), enemy_type, rng));
        }

        let mut next_floor_loot: Vec<Loot> = vec![];
        for _ in 0..2 {
            next_floor_loot.push(Loot::generate_loot(*level, rng));
        }

        Floor::new(enemys, loot, next_floor_loot)
    }
}

#[derive(Debug, Clone, Copy)]
enum LootType {
    MiB,
    Upgrade,
    GithubTokens
}


#[derive(Debug)]
struct Loot {
    loot_type: LootType,
    loot_amount: Option<u32>
}

impl Loot {
    fn new(loot_type: LootType, loot_amount: Option<u32>) -> Self {
        Self {
            loot_type,
            loot_amount
        }
    }

    fn generate_loot(level: u32, rng: &mut ThreadRng) -> Self {
        let loot_types = [LootType::MiB, LootType::GithubTokens, LootType::Upgrade];
        let loot_type = loot_types[rng.gen_range(0..loot_types.len()) as usize];
        let loot_amount = match loot_type {
            LootType::Upgrade => None,
            _ => Some(((level*10) as i32 + rng.gen_range(-5..5)) as u32)
        };

        Loot::new(loot_type, loot_amount)
    }
}

#[derive(Debug, Clone, Copy)]
enum EnemyType {
    Tank, // .iso
    Supporter, // .dll
    Spawner, // .zip
    DamageDealer, // .exe
    //Boss, // .bin
    Debuffer // .rs
}

#[derive(Debug)]
struct Enemy {
    health: i32,
    damage: i32,
    level: u32, // scaling for the health and damage
    enemy_type: EnemyType
    // ToDo: add some parameters, when building the AI for the enemys
}

impl Enemy {
    fn new(health: i32, damage: i32, level: u32, enemy_type: EnemyType) -> Self {
        Self {
            health,
            damage,
            level,
            enemy_type
        }
    }

    fn generate_enemy(level: &u32, enemy_type: EnemyType, rng: &mut ThreadRng) -> Self {
        let mut base_health: u32;
        let mut base_damage: u32;
        match enemy_type {
            /* 
            EnemyType::Boss => {
                base_health = 600;
                base_damage = 100;
            },
            */
            EnemyType::DamageDealer => {
                base_health = 70;
                base_damage = 125;
            },
            EnemyType::Debuffer => {
                base_health = 125;
                base_damage = 40;
            },
            EnemyType::Spawner => {
                base_health = 100;
                base_damage = 20;
            },
            EnemyType::Supporter => {
                base_health = 125;
                base_damage = 40;
            },
            EnemyType::Tank => {
                base_health = 250;
                base_damage = 40;
            }
        };

        let health = ((base_health as i32 + (rng.gen_range(-3..=3)*5)) as f64 * (1.0 + *level as f64 / 20.0)).round() as i32;
        let damage = ((base_damage as i32 + (rng.gen_range(-3..=3)*2)) as f64 * (1.0 + *level as f64 / 20.0)).round() as i32;

        Enemy::new(health, damage, *level, enemy_type)
    }
}

#[allow(unused)]
fn main() {
    let mut rng = rand::thread_rng();

    let mut level = 1;
    let loot = Loot::generate_loot(level, &mut rng);
    let floor = Floor::generate_floor(&level, loot, &mut rng);

    println!("{:#?}", floor);
    /*
    let floor = Floor::new(
        vec![Enemy::generate_enemy(&level, EnemyType::DamageDealer, &mut rng), Enemy::generate_enemy(&level, EnemyType::Tank, &mut rng), Enemy::generate_enemy(&level, EnemyType::Supporter, &mut rng)],
        Loot::new("MiB".to_string(), Some(10)),
        vec![Loot::new("Mib".to_string(), Some(20)), Loot::new("Upgrade".to_string(), None)]
    );
    */
}
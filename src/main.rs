#![allow(unused)]
use rand::{Rng, rngs::ThreadRng};

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
}

struct Loot {
    reward_type: String,
    reward_amount: Option<u32>
}

impl Loot {
    fn new(reward_type: String, reward_amount: Option<u32>) -> Self {
        Self {
            reward_type,
            reward_amount
        }
    }
}

enum EnemyType {
    Tank, // .iso
    Supporter, // .dll
    Spawner, // .zip
    DamageDealer, // .exe
    Boss, // .bin
    Debuffer // .rs
}


struct Enemy {
    health: u32,
    damage: u32,
    level: u32, // scaling for the health and damage
    enemy_type: EnemyType
    // ToDo: add some parameters, when building the AI for the enemys
}

impl Enemy {
    fn new(health: u32, damage: u32, level: u32, enemy_type: EnemyType) -> Self {
        Self {
            health,
            damage,
            level,
            enemy_type
        }
    }

    fn generate_enemy(level: u32, enemy_type: EnemyType, rng: ThreadRng) -> Self {
        let mut base_health: u32;
        let mut base_damage: u32;
        match enemy_type {
            EnemyType::Boss => {
                base_health = 600;
                base_damage = 100;
            },
            EnemyType::DamageDealer => {
                base_health = 70;
                base_damage = 125;
            },
            EnemyType::Debuffer => {
                bas
            },
            EnemyType::Spawner => {

            },
            EnemyType::Supporter => {

            },
            EnemyType::Tank => {

            }
        }
    }
}

#[allow(unused)]
fn main() {
    let mut rng = rand::thread_rng();

    let floor = Floor::new(
        vec![Enemy::new(100, 200, 1, EnemyType::DamageDealer), Enemy::new(200, 50, 1, EnemyType::Tank), Enemy::new(125, 20, 1, EnemyType::Supporter)],
        Loot::new("MiB".to_string(), Some(10)),
        vec![Loot::new("Mib".to_string(), Some(20)), Loot::new("Upgrade".to_string(), None)]
    );
}
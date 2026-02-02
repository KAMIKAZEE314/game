#![allow(unused)]
use rand::{Rng, rngs::ThreadRng};

pub mod player;

#[derive(Debug)]
pub struct Floor {
    enemys: Vec<Enemy>,
    loot: Loot,
    next_floor_loot: Vec<Loot>
}

impl Floor {
    pub fn new(enemys: Vec<Enemy>, loot: Loot, next_floor_loot: Vec<Loot>) -> Self {
        Self {
            enemys,
            loot,
            next_floor_loot
        }
    }

    pub fn get_enemys(&mut self) -> &mut [Enemy] {&mut self.enemys}

    pub fn generate_floor(level: &u32, loot: Loot, mut rng: &mut ThreadRng) -> Self {
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
pub enum LootType {
    MiB,
    Upgrade,
    GithubTokens
}


#[derive(Debug)]
pub struct Loot {
    loot_type: LootType,
    loot_amount: Option<u32>
}

impl Loot {
    pub fn new(loot_type: LootType, loot_amount: Option<u32>) -> Self {
        Self {
            loot_type,
            loot_amount
        }
    }

    pub fn generate_loot(level: u32, rng: &mut ThreadRng) -> Self {
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
pub enum EnemyType {
    Tank, // .iso
    Supporter, // .dll
    Spawner, // .zip
    DamageDealer, // .exe
    //Boss, // .bin
    Debuffer // .rs
}

#[derive(Debug)]
pub struct Enemy {
    id: u32,
    health: u32,
    damage: u32,
    level: u32, // scaling for the health and damage
    enemy_type: EnemyType
    // ToDo: add some parameters, when building the AI for the enemys
}

impl Enemy {
    pub fn new(id: u32, health: u32, damage: u32, level: u32, enemy_type: EnemyType) -> Self {
        Self {
            id,
            health,
            damage,
            level,
            enemy_type
        }
    }

    pub fn get_id(&self) -> &u32 {&self.id}

    pub fn generate_enemy(level: &u32, enemy_type: EnemyType, rng: &mut ThreadRng) -> Self {
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

        let health = ((base_health as i32 + (rng.gen_range(-3..=3)*5)) as f64 * (1.0 + *level as f64 / 20.0)).round() as u32;
        let damage = ((base_damage as i32 + (rng.gen_range(-3..=3)*2)) as f64 * (1.0 + *level as f64 / 20.0)).round() as u32;

        Enemy::new(health * damage * *level * rng.gen_range(0..=100000), health, damage, *level, enemy_type)
    }

    pub fn compare_id(&self, id: &u32) -> bool {
        if self.id == *id {
            true
        } else {
            false
        }
    } 
}
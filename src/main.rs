#![allow(unused)]
use rand::{Rng, rngs::ThreadRng};

pub mod core;

#[allow(unused)]
fn main() {
    let mut rng = rand::thread_rng();

    let mut level = 1;
    let loot = core::Loot::generate_loot(level, &mut rng);
    let floor = core::Floor::generate_floor(&level, loot, &mut rng);

    println!("{:#?}", floor);
    /*
    let floor = Floor::new(
        vec![Enemy::generate_enemy(&level, EnemyType::DamageDealer, &mut rng), Enemy::generate_enemy(&level, EnemyType::Tank, &mut rng), Enemy::generate_enemy(&level, EnemyType::Supporter, &mut rng)],
        Loot::new("MiB".to_string(), Some(10)),
        vec![Loot::new("Mib".to_string(), Some(20)), Loot::new("Upgrade".to_string(), None)]
    );
    */
}
#![allow(unused)]
use std::vec;

use rand::{Rng, rngs::ThreadRng};

pub mod core;

#[allow(unused)]
fn main() {
    let ATTACKS: [core::player::Attack; 1] = [core::player::Attack::new("sudo rm -fr /", 99999, 3, 1, 1)];
    let mut rng = rand::thread_rng();

    let mut level = 1;
    let loot = core::Loot::generate_loot(level, &mut rng);
    let floor = core::Floor::generate_floor(&level, loot, &mut rng);

    let mut player = core::player::Player::new(ATTACKS.to_vec(), vec![], 0, 100, 1, Some(floor), 0, 0);
    println!("{:#?}", player);

    let attack = player.get_attack_set()[0];
    player.equip_attacks(&[attack]);

    println!("{:#?}", player);

    let current_floor: &mut core::Floor = match player.get_current_floor() {
        Some(floor) => floor,
        None => panic!("No floor available!"),
    };


    let attack_targets = current_floor.get_enemys();

    let mut attack_target_ids: Vec<u32> = vec![];
    for attack_target in attack_targets {
        attack_target_ids.push(*(attack_target.get_id()));
    }
    let attack_target_ids: &[u32] = &attack_target_ids;

    let equipped_attack = player.get_equipped_attacks()[0];
    let mut ap = *player.get_ap();

    player.attack(&equipped_attack, attack_target_ids, &ap);
    

    println!("{:#?}", player);
}
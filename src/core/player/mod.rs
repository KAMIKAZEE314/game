#![allow(unused)]

use crate::core::{self, Floor};

#[derive(Debug)]
pub struct Player {
    attack_set: Vec<Attack>,
    equipped_attacks: Vec<Attack>,
    level: u32,
    memory: u32,
    ap: u32,
    current_floor: Option<core::Floor>,
    mib: u32, // Currency for tempory upgrades
    github_tokens: u32, // Currency for permanent upgrades

}

impl Player {
    pub fn new(attack_set: Vec<Attack>, equipped_attacks: Vec<Attack>, level: u32, memory: u32, ap: u32, current_floor: Option<core::Floor>, mib: u32, github_tokens: u32) -> Self {
        Self {
            attack_set,
            equipped_attacks,
            level,
            memory,
            ap,
            current_floor,
            mib,
            github_tokens
        }
    }

    pub fn get_attack_set(&self) -> &[Attack] {&self.attack_set}
    pub fn get_equipped_attacks(&self) -> &[Attack] {&self.equipped_attacks}
    pub fn get_ap(&self) -> &u32 {&self.ap}
    pub fn get_current_floor(&mut self) -> Option<&mut Floor> { self.current_floor.as_mut()}
    pub fn add_ap(&mut self, ap: u32) {self.ap += ap;}
    pub fn add_memory(&mut self, memory: u32) {self.memory += memory;}
    pub fn add_mib(&mut self, mib: u32) {self.mib += mib;}
    pub fn add_github_tokens(&mut self, github_tokens: u32) {self.github_tokens += github_tokens;}
    pub fn remove_ap(&mut self, ap: u32) {self.ap -= ap;}
    pub fn remove_memory(&mut self, memory: u32) {self.memory -= memory;}
    pub fn remove_mib(&mut self, mib: u32) {self.mib -= mib;}
    pub fn remove_github_tokens(&mut self, github_tokens: u32) {self.github_tokens -= github_tokens;}
    pub fn set_floor(&mut self, floor: Option<Floor>) {self.current_floor = floor;}
    pub fn push_attack_set(&mut self, attack: Attack) {self.attack_set.push(attack);}

    pub fn equip_attacks(&mut self, attacks: &[Attack]) -> Result<i32, &'static str> { // In case of Ok(), we won't return an int with a meaning
        let mut result: Result<i32, &'static str> = Err("Slice of attacks musn't be 0!"); 
        for attack in attacks {
            if (*attack).memory_size <= self.memory {
                self.equipped_attacks.push(*attack);
                self.remove_memory((*attack).memory_size);
                result = Ok(1);
            } else {
                result = Err("Not enough memory avaliable!");
            }
        };
        result
    }

    pub fn attack(&mut self, attack: &Attack, attack_targets: &[u32], ap: &u32) -> Result<u32, &'static str> {
        if attack.ap_cost <= *ap {
            let num_attack_targets = attack.get_num_targets();
            let num_attacked_targets: u32 = 0;
            for attack_target in attack_targets {
                match &mut self.current_floor {
                    Some(floor) => {
                        for enemy in floor.get_enemys() {
                            if (*enemy).compare_id(attack_target) {
                                if attack.damage > (*enemy).health {
                                    (*enemy).health = 0;
                                } else {
                                    (*enemy).health -= attack.damage;
                                }
                                break;
                            }
                        }
                    },
                    None => return Err("There is no Floor!")
                }
                if num_attacked_targets == *num_attack_targets {
                    break;
                }
            }

            self.remove_ap(attack.ap_cost);
        } else {
            return Err("Not enough AP!");
        }
        Ok(1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    name: &'static str,
    damage: u32,
    num_targets: u32,
    ap_cost: u32,
    memory_size: u32
}

impl Attack {
    pub fn new(name: &'static str, damage: u32, num_targets: u32, ap_cost: u32, memory_size: u32) -> Self {
        Self {
            name,
            damage,
            num_targets,
            ap_cost,
            memory_size
        }
    }

    pub fn get_num_targets(&self) -> &u32 {&self.num_targets}
}

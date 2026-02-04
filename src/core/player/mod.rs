#![allow(unused)]

use std::env::consts::FAMILY;
use std::{char, fs, num, vec};
use std::fmt::format;

use crate::core::{self, Enemy, Floor, Loot, LootType};

#[derive(Debug)]
pub struct Player {
    name: String,
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
    pub fn new(name: String, attack_set: Vec<Attack>, equipped_attacks: Vec<Attack>, level: u32, memory: u32, ap: u32, current_floor: Option<core::Floor>, mib: u32, github_tokens: u32) -> Self {
        Self {
            name,
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

    pub fn get_name(&self) -> &String {&self.name}
    pub fn get_attack_set(&self) -> &[Attack] {&self.attack_set}
    pub fn get_equipped_attacks(&self) -> &[Attack] {&self.equipped_attacks}
    pub fn get_level(&self) -> &u32 {&self.level}
    pub fn get_memory(&self) -> &u32 {&self.memory}
    pub fn get_ap(&self) -> &u32 {&self.ap}
    pub fn get_mut_current_floor(&mut self) -> Option<&mut Floor> { self.current_floor.as_mut()}
    pub fn get_current_floor(&self) -> Option<&Floor> {self.current_floor.as_ref()}
    pub fn get_mib(&self) -> &u32 {&self.mib}
    pub fn get_github_tokens(&self) -> &u32 {&self.github_tokens}
    pub fn add_ap(&mut self, ap: u32) {self.ap += ap;}
    pub fn add_memory(&mut self, memory: u32) {self.memory += memory;}
    pub fn add_mib(&mut self, mib: u32) {self.mib += mib;}
    pub fn add_github_tokens(&mut self, github_tokens: u32) {self.github_tokens += github_tokens;}
    pub fn remove_ap(&mut self, ap: u32) {self.ap -= ap;}
    pub fn remove_memory(&mut self, memory: u32) {self.memory -= memory;}
    pub fn remove_mib(&mut self, mib: u32) {self.mib -= mib;}
    pub fn remove_github_tokens(&mut self, github_tokens: u32) {self.github_tokens -= github_tokens;}
    pub fn set_floor(&mut self, floor: Option<Floor>) {self.current_floor = floor;}
    pub fn push_attack_set(&mut self, attacks: &[Attack]) {
        for attack in attacks {
            self.attack_set.push(attack.clone());
        }
    }

    pub fn equip_attacks(&mut self, attacks: &[Attack]) -> Result<(), &'static str> { // In case of Ok(), we won't return an int with a meaning
        let mut result: Result<(), &'static str> = Err("Slice of attacks musn't be 0!"); 
        for attack in attacks {
            if (*attack).memory_size <= self.memory {
                self.equipped_attacks.push(attack.clone());
                self.remove_memory((*attack).memory_size);
                result = Ok(());
            } else {
                result = Err("Not enough memory avaliable!");
            }
        };
        result
    }

    pub fn attack(&mut self, attack: &Attack, attack_targets: &[u32], ap: &u32) -> Result<(), &'static str> {
        if attack.ap_cost <= *ap {
            let num_attack_targets = attack.get_num_targets();
            let num_attacked_targets: u32 = 0;
            for attack_target in attack_targets {
                match &mut self.current_floor {
                    Some(floor) => {
                        for enemy in floor.get_mut_enemys() {
                            if (*enemy).compare_id(attack_target) {
                                if attack.damage >= (*enemy).health {
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
        Ok(())
    }

    pub fn save_player(&self, file: &'static str) {
        let mut information_string = String::new();

        information_string.push_str(&format!("{}\n", self.name));

        for attack in self.get_attack_set() {
            information_string.push_str(&format!("{}!", attack.get_text_format()));
        }
        information_string.push_str("\n");

        for attack in self.get_equipped_attacks() {
            information_string.push_str(&format!("{}!", attack.get_text_format()));
        }
        information_string.push_str("\n");

        information_string.push_str(&format!("{}\n", self.level));
        information_string.push_str(&format!("{}\n", self.memory));
        information_string.push_str(&format!("{}\n", self.ap));

        match &self.current_floor {
            Some(floor) => {
                information_string.push_str("S ");

                information_string.push_str(&floor.get_text_format());
                information_string.push_str("\n");
            },
            None => {information_string.push_str("N\n");}
        }

        information_string.push_str(&format!("{}\n", self.mib));
        information_string.push_str(&format!("{}", self.github_tokens));
 
        fs::write(file, information_string);
    }

    pub fn load_player(file: &'static str) -> Self {
        let content = fs::read_to_string(file).expect(&format!("Couldn't read \"{}\"!", file));
        let mut content: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        let mut name: String = content[0].clone();

        let mut attack_set: Vec<Attack> = vec![];
        let mut attack: Vec<String> = vec![];
        let mut attack_piece: String = String::new();
        for char in content[1].chars() {
            match char {
                ',' => {
                    attack.push(attack_piece);
                    attack_piece = String::new();
                },
                '!' => {
                    if attack_piece != String::new() {
                        attack.push(attack_piece);
                        attack_piece = String::new();
                    }

                    attack_set.push(Attack::new(attack[0].clone(), attack[1].parse().expect("Invalid Save data!"), attack[2].parse().expect("Invalid Save data!"), attack[3].parse().expect("Invalid Save data!"), attack[4].parse().expect("Invalid Save data!")));
                },
                _ => {
                    attack_piece.push(char);
                }
            }
        }

        let mut equipped_attacks: Vec<Attack> = vec![];
        let mut attack: Vec<String> = vec![];
        let mut attack_piece: String = String::new();
        for char in content[2].chars() {
            match char {
                ',' => {
                    attack.push(attack_piece);
                    attack_piece = String::new();
                },
                '!' => {
                    if attack_piece != String::new() {
                        attack.push(attack_piece);
                        attack_piece = String::new();
                    }

                    equipped_attacks.push(Attack::new(attack[0].clone(), attack[1].parse().expect("Invalid Save data!"), attack[2].parse().expect("Invalid Save data!"), attack[3].parse().expect("Invalid Save data!"), attack[4].parse().expect("Invalid Save data!")));
                },
                _ => {
                    attack_piece.push(char);
                }
            }
        }

        let mut level: u32 = content[3].parse().expect("Invalid Save data!");

        let mut memory: u32 = content[4].parse().expect("Invalid Save data!");

        let mut ap: u32 = content[5].parse().expect("Invalid Save data!");

        let mut rng = rand::thread_rng();
        let mut current_floor: Option<core::Floor> = None;
        if content[6].chars().nth(0) == Some('N') {
            current_floor = None;
        } else {
            let mut enemys: Vec<Enemy> = vec![];
            let mut enemy: Vec<String> = vec![];
            let mut enemy_piece = String::new();
            let mut next_floor_loot: Vec<Loot> = vec![];
            let mut floor_loot: Loot = Loot::new(LootType::GithubTokens, Some(1));
            let mut loot: Vec<String> = vec![];
            let mut loot_piece = String::new();
            let mut num_seperators: u32 = 0;
            for char in content[6].chars().skip(2) {
                match char {
                    ',' => {
                        match num_seperators {
                            0 => {
                                enemy.push(enemy_piece);
                                enemy_piece = String::new();
                            },
                            1 | 2 => {
                                loot.push(loot_piece);
                                loot_piece = String::new();
                            },
                            _ => panic!("Invalid Save Data (Too many §-Seperators)!")
                        }
                    },
                    '!' => {
                        enemy.push(enemy_piece);
                        enemy_piece = String::new();
                        let enemy_type = match enemy[4].as_str() {
                            "Da" => core::EnemyType::DamageDealer,
                            "De" => core::EnemyType::Debuffer,
                            "Sp" => core::EnemyType::Spawner,
                            "Su" => core::EnemyType::Supporter,
                            "Ta" => core::EnemyType::Tank,
                            _ => panic!("Invalid Enemy Type!")
                        };
                        enemys.push(Enemy::new(enemy[0].parse().expect("Invalid Save Data!"), enemy[1].parse().expect("Invalid Save Data!"), enemy[2].parse().expect("Invalid Save Data!"), enemy[3].parse().expect("Invalid Save Data!"), enemy_type));
                        enemy = vec![];
                    },
                    '§' => {
                        match num_seperators {
                            0 | 3 => {},
                            1 => {
                                loot.push(loot_piece);
                                loot_piece = String::new();

                                let loot_type = match loot[0].as_str() {
                                    "M" => LootType::MiB,
                                    "G" => LootType::GithubTokens,
                                    "U" => LootType::Upgrade,
                                    _ => panic!("Invalid Save Data!")
                                };
                                let mut loot_amount: Option<u32>;
                                match loot[1].chars().nth(0) {
                                    Some(char) => {
                                        match char {
                                            'N' => {
                                                loot_amount = None;
                                            },
                                            'S' => {
                                                loot_amount = Some(loot[1].chars().skip(2).collect::<String>().parse::<u32>().expect("Invalid Save Data!"));
                                            },
                                            _ => panic!("Invalid Save Data!")
                                        }
                                    },
                                    None => panic!("Invalid Save Data!")
                                }

                                floor_loot = Loot::new(loot_type, loot_amount)
                            },
                            2 => {
                                loot.push(loot_piece);
                                loot_piece = String::new();

                                let loot_type = match loot[0].as_str() {
                                    "M" => LootType::MiB,
                                    "G" => LootType::GithubTokens,
                                    "U" => LootType::Upgrade,
                                    _ => panic!("Invalid Save Data!")
                                };
                                let mut loot_amount: Option<u32>;
                                match loot[1].chars().nth(0) {
                                    Some(char) => {
                                        match char {
                                            'N' => {
                                                loot_amount = None;
                                            },
                                            'S' => {
                                                loot_amount = Some(loot[1].chars().skip(2).collect::<String>().parse::<u32>().expect("Invalid Save Data!"));
                                            },
                                            _ => panic!("Invalid Save Data!")
                                        }
                                    },
                                    None => panic!("Invalid Save Data!")
                                }
                                next_floor_loot.push(Loot::new(loot_type, loot_amount));

                                let loot_type = match loot[2].as_str() {
                                    "M" => LootType::MiB,
                                    "G" => LootType::GithubTokens,
                                    "U" => LootType::Upgrade,
                                    _ => panic!("Invalid Save Data!")
                                };
                                let mut loot_amount: Option<u32>;
                                match loot[3].chars().nth(0) {
                                    Some(char) => {
                                        match char {
                                            'N' => {
                                                loot_amount = None;
                                            },
                                            'S' => {
                                                loot_amount = Some(loot[3].chars().skip(2).collect::<String>().parse::<u32>().expect("Invalid Save Data!"));
                                            },
                                            _ => panic!("Invalid Save Data!")
                                        }
                                    },
                                    None => panic!("Invalid Save Data!")
                                }
                                next_floor_loot.push(Loot::new(loot_type, loot_amount));
                            },
                            _ => panic!("Invalid Save Data (Too many §-Seperators)!")
                        }
                        num_seperators += 1;
                    },
                    _ => {
                        match num_seperators {
                            0 => enemy_piece.push(char),
                            1 | 2 => {
                                loot_piece.push(char);
                            },
                            _ => panic!("Invalid Save Data (Too many §-Seperators)!")
                        }
                    }
                }
            }       
            current_floor = Some(Floor::new(enemys, floor_loot, next_floor_loot));
        }

        let mut mib: u32 = content[7].parse().expect("Invalid Save data!");

        let mut github_tokens: u32 = content[8].parse().expect("Invalid Save data!");

        Player::new(name, attack_set, equipped_attacks, level, memory, ap, current_floor, mib, github_tokens)
    }
}

#[derive(Clone, Debug)]
pub struct Attack {
    name: String,
    damage: u32,
    num_targets: u32,
    ap_cost: u32,
    memory_size: u32
}

impl Attack {
    pub fn new(name: String, damage: u32, num_targets: u32, ap_cost: u32, memory_size: u32) -> Self {
        Self {
            name,
            damage,
            num_targets,
            ap_cost,
            memory_size
        }
    }

    pub fn get_num_targets(&self) -> &u32 {&self.num_targets}
    pub fn get_text_format(&self) -> String {
        String::from(format!("{},{},{},{},{}", self.name, self.damage, self.num_targets, self.ap_cost, self.memory_size))
    }
}

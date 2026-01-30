struct Floor {
    enemys: vec<Enemy>,
    loot: Loot,
    next_floor_loot: vec<Loot>
}

struct Loot {
    reward_type: String,
    reward_amount: i32
}

struct Enemy {
    health: u32
    // ToDo: add some parameters, when building the AI for the enemys
}

#[allow(unused)]
fn main() {
    let floor = 
}
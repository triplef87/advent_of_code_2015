#[derive(Debug, Copy, Clone)]
struct State {
    hp: usize,
    damage: usize,
    armor: usize
}

impl State {
    fn be_attacked(&mut self, attack: usize) -> bool {
        let damage = if attack > self.armor { attack - self.armor } else { 1 };

        if damage >= self.hp { 
            self.hp = 0;
            true
        } else {
            self.hp = self.hp - damage;
            false
        }
    }

    fn equip(&self, mut items: usize) -> State {
        let ring_1 = items % 10;
        items = items / 10;
    
        let ring_2 = items % 10;
        items = items / 10;
    
        let armor = items % 10;
        items = items / 10;
    
        let weapon = items % 10;

        let mut new = self.clone();

        new.damage = new.damage + WEAPONS[weapon - 1].damage;

        if armor > 0 {
            new.armor = new.armor + ARMORS[armor - 1].armor;
        }

        if ring_1 > 0 {
            new.damage = new.damage + RINGS[ring_1 - 1].damage;
            new.armor = new.armor + RINGS[ring_1 - 1].armor;
        }

        if ring_2 > 0 {
            new.damage = new.damage + RINGS[ring_2 - 1].damage;
            new.armor = new.armor + RINGS[ring_2 - 1].armor;
        }

        new
    }
}

struct Item {
    cost: usize,
    damage: usize,
    armor: usize
}

static WEAPONS: [Item; 5] = [
        Item { cost: 8, damage: 4, armor: 0 },
        Item { cost: 10, damage: 5, armor: 0 },
        Item { cost: 25, damage: 6, armor: 0 },
        Item { cost: 40, damage: 7, armor: 0 },
        Item { cost: 74, damage: 8, armor: 0 },
        ];
    
static ARMORS: [Item; 5] =[
    Item { cost: 13, damage: 0, armor: 1 },
    Item { cost: 31, damage: 0, armor: 2 },
    Item { cost: 53, damage: 0, armor: 3 },
    Item { cost: 75, damage: 0, armor: 4 },
    Item { cost: 102, damage: 0, armor: 5 },
    ];

static RINGS: [Item; 6] = [
    Item { cost: 25, damage: 1, armor: 0 },
    Item { cost: 50, damage: 2, armor: 0 },
    Item { cost: 100, damage: 3, armor: 0 },
    Item { cost: 20, damage: 0, armor: 1 },
    Item { cost: 40, damage: 0, armor: 2 },
    Item { cost: 80, damage: 0, armor: 3 },
    ];

fn main() {
    let mut boss = State{ hp: 104, damage: 8, armor: 1 };
    let player = State { hp: 100, damage: 0, armor: 0 };

    let mut items = 1000;
    let mut item_combinations: Vec<usize> = Vec::new();

    while items < 5566 {
        if get_value(items) > 0 {
           item_combinations.push(items);
        }
        items = items + 1;
    }

    println!("{:?}", item_combinations);

    let mut max = 0;
    for val in item_combinations {
        let mut equip_player = player.equip(val);
        let cost = get_value(val);
        println!("{} -> {}", val, cost);
        if !fight(&mut boss.clone(), &mut equip_player) && cost > max {
            max = cost;
        }
    }

    println!("{}", max);

    // let mut equip_player = player.equip(2100);
    // fight(&mut boss, &mut equip_player);
    
}

fn fight(boss: &mut State, player: &mut State) -> bool {
    loop {
        if boss.be_attacked(player.damage) { return true; };
        if player.be_attacked(boss.damage) { return false; };
        println!("{:?}, {:?}", boss, player);
    }
}

fn get_value(mut items: usize) -> usize {
    let mut cost = 0;

    let ring_1 = items % 10;
    if ring_1 > 6 { return  0; }
    items = items / 10;

    let ring_2 = items % 10;
    if ring_2 > 6 || ring_1 <= ring_2 { return  0; }
    items = items / 10;

    let armor = items % 10;
    if armor > 5 { return  0; }
    items = items / 10;

    let weapon = items % 10;

    cost = cost + WEAPONS[weapon - 1].cost;

    if armor > 0 {
        cost = cost + ARMORS[armor - 1].cost;
    }

    if ring_1 > 0 {
        cost = cost + RINGS[ring_1 - 1].cost;
    }

    if ring_2 > 0 {
        cost = cost + RINGS[ring_2 - 1].cost;
    }

    cost
}

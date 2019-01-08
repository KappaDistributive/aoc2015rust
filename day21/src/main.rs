#[derive(Clone)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(Clone)]
struct Item {
    name: String,
    itemtype: ItemType,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    fn from_data(
        name: String,
        itemtype: ItemType,
        cost: usize,
        damage: usize,
        armor: usize,
    ) -> Self {
        Item {
            name,
            itemtype,
            cost,
            damage,
            armor,
        }
    }
}
struct Shop {
    inventory: Vec<Item>,
}

impl Shop {
    fn init() -> Self {
        let mut inventory: Vec<Item> = Vec::new();

        // add weapons

        // add armor

        // add rings

        Shop { inventory }
    }
}

#[derive(Clone)]
struct Warrior {
    hitpoints: usize,
    damage: usize,
    armor: usize,
    items: Vec<Item>,
}

impl Warrior {
    fn from_stats(hitpoints: usize, damage: usize, armor: usize) -> Self {
        Warrior {
            hitpoints,
            damage,
            armor,
            items: Vec::new(),
        }
    }

    fn from_items(items: Vec<Item>) -> Self {
        let hitpoints: usize = 100;
        let mut damage: usize = 0;
        let mut armor: usize = 0;

        for item in items.clone() {
            damage += item.damage;
            armor += item.armor;
        }

        Warrior {
            hitpoints,
            damage,
            armor,
            items,
        }
    }

    fn item_value(&self) -> usize {
        let mut value: usize = 0;
        for item in self.items.clone() {
            value += item.cost;
        }
        value
    }

    fn fight_round(&mut self, opponent: &Warrior) {
        if opponent.damage > self.armor {
            if self.hitpoints > opponent.damage - self.armor {
                self.hitpoints -= opponent.damage - self.armor;
            } else {
                self.hitpoints = 0;
            }
        } else {
            if self.hitpoints > 0 {
                self.hitpoints -= 1;
            } else {
                self.hitpoints = 0;
            }
        }
    }
}

fn init() -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    let mut weapons: Vec<Item> = Vec::new();
    weapons.push(Item::from_data(
        String::from("Dagger"),
        ItemType::Weapon,
        8,
        4,
        0,
    ));
    weapons.push(Item::from_data(
        String::from("Shortsword"),
        ItemType::Weapon,
        10,
        5,
        0,
    ));
    weapons.push(Item::from_data(
        String::from("Warhammer"),
        ItemType::Weapon,
        25,
        6,
        0,
    ));
    weapons.push(Item::from_data(
        String::from("Longsword"),
        ItemType::Weapon,
        40,
        7,
        0,
    ));
    weapons.push(Item::from_data(
        String::from("Greataxe"),
        ItemType::Weapon,
        74,
        8,
        0,
    ));

    let mut armor: Vec<Item> = Vec::new();
    armor.push(Item::from_data(
        String::from("Leather"),
        ItemType::Armor,
        13,
        0,
        1,
    ));
    armor.push(Item::from_data(
        String::from("Chainmail"),
        ItemType::Armor,
        31,
        0,
        2,
    ));
    armor.push(Item::from_data(
        String::from("Splintmail"),
        ItemType::Armor,
        53,
        0,
        3,
    ));
    armor.push(Item::from_data(
        String::from("Bandedmail"),
        ItemType::Armor,
        75,
        0,
        4,
    ));
    armor.push(Item::from_data(
        String::from("Platemail"),
        ItemType::Armor,
        102,
        0,
        5,
    ));

    let mut rings: Vec<Item> = Vec::new();
    rings.push(Item::from_data(
        String::from("Damage +1"),
        ItemType::Ring,
        25,
        1,
        0,
    ));
    rings.push(Item::from_data(
        String::from("Damage +2"),
        ItemType::Ring,
        50,
        2,
        0,
    ));
    rings.push(Item::from_data(
        String::from("Damage +3"),
        ItemType::Ring,
        100,
        3,
        0,
    ));
    rings.push(Item::from_data(
        String::from("Defense +1"),
        ItemType::Ring,
        20,
        0,
        1,
    ));
    rings.push(Item::from_data(
        String::from("Defense +2"),
        ItemType::Ring,
        40,
        0,
        2,
    ));
    rings.push(Item::from_data(
        String::from("Defense +3"),
        ItemType::Ring,
        80,
        0,
        3,
    ));
    (weapons, armor, rings)
}
// returns true iff player wins
fn fight_to_death(player: &Warrior, enemy: &Warrior) -> bool {
    let mut player_t = player.clone();
    let mut enemy_t = enemy.clone();
    loop {
        enemy_t.fight_round(player);
        if enemy_t.hitpoints == 0 {
            return true;
        }
        player_t.fight_round(enemy);
        if player_t.hitpoints == 0 {
            return false;
        }
    }
    unreachable!();
}

fn solve_part_1() -> usize {
    let mut victors: Vec<Warrior> = Vec::new();
    let (weapons, armor, rings) = init();
    let enemy: Warrior = Warrior::from_stats(103, 9, 2);

    for weapon in weapons {
        for i in 0..armor.len() + 1 {
            for j in 0..rings.len() + 1 {
                for k in j..rings.len() + 1 {
                    let mut items: Vec<Item> = Vec::new();
                    items.push(weapon.clone());
                    if i < armor.len() {
                        items.push(armor[i].clone());
                    }
                    if j < rings.len() {
                        items.push(rings[j].clone());
                    }
                    if k != j && k < rings.len() {
                        items.push(rings[k].clone());
                    }
                    let player: Warrior = Warrior::from_items(items);
                    if fight_to_death(&player, &enemy) {
                        victors.push(player);
                    }
                }
            }
        }
    }
    let mut answer: usize = std::usize::MAX;
    for victor in victors {
        answer = std::cmp::min(answer, victor.item_value());
    }
    answer
}

fn solve_part_2() -> usize {
    let mut losers: Vec<Warrior> = Vec::new();
    let (weapons, armor, rings) = init();
    let enemy: Warrior = Warrior::from_stats(103, 9, 2);

    for weapon in weapons {
        for i in 0..armor.len() + 1 {
            for j in 0..rings.len() + 1 {
                for k in j..rings.len() + 1 {
                    let mut items: Vec<Item> = Vec::new();
                    items.push(weapon.clone());
                    if i < armor.len() {
                        items.push(armor[i].clone());
                    }
                    if j < rings.len() {
                        items.push(rings[j].clone());
                    }
                    if k != j && k < rings.len() {
                        items.push(rings[k].clone());
                    }
                    let player: Warrior = Warrior::from_items(items);
                    if !fight_to_death(&player, &enemy) {
                        losers.push(player);
                    }
                }
            }
        }
    }
    let mut answer: usize = 0;
    for loser in losers {
        answer = std::cmp::max(answer, loser.item_value());
    }
    answer
}

fn main() {
    println!("Answer part 1: {}", solve_part_1());
    println!("Answer part 2: {}", solve_part_2());
}

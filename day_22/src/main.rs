#[derive(Debug, Copy, Clone)]
struct Boss {
    hp: usize,
    damage: usize,
    poison: usize
}

impl Boss {
    fn turn(&mut self, mut player: Player) -> Player {
        if player.shield > 0 { player.shield = player.shield - 1; }
        if player.recharge > 0 { 
            player.recharge = player.recharge - 1;
            player.mana = player.mana + 101;
        }

        if self.poison > 0 {
            self.hp = self.hp.saturating_sub(3);
            self.poison = self.poison - 1;
        }

        if self.hp == 0 {
            return player;
        }

        let hit = if player.shield == 0 { self.damage } else { self.damage - 7 };
        player.hp = player.hp.saturating_sub(hit);

        player
    }
}

#[derive(Debug, Copy, Clone)]
struct Player {
    hp: usize,
    mana: usize,
    shield: usize,
    recharge: usize
}

impl Player {
    fn turn(&mut self, mut boss: Boss, mut spell: usize) -> (Boss, usize) {
        self.hp = self.hp - 1;
        if self.hp == 0 { return (boss, 0);}


        if self.shield > 0 { self.shield = self.shield - 1 }
        if self.recharge > 0 {
            self.mana = self.mana + 101;
            self.recharge = self.recharge - 1;
        }

        if boss.poison > 0 {
            boss.hp = boss.hp.saturating_sub(3);
            boss.poison = boss.poison - 1;
        }

        if boss.hp == 0 {
            spell = 0;
        }

        let mut cast: usize = 300;
        match spell {
            1 => {
                if self.mana >= 53 {
                    cast = 53;
                    self.mana = self.mana - 53;
                    boss.hp = boss.hp.saturating_sub(4);
                }
            },
            2 => {
                if self.mana >= 73 {
                    cast = 73;
                    self.mana = self.mana - 73;
                    boss.hp = boss.hp.saturating_sub(2);
                    self.hp = self.hp + 2;
                }
            },
            3 => {
                if self.mana >= 113 && self.shield == 0 {
                    cast = 113;
                    self.mana = self.mana - 113;
                    self.shield = 6;
                }
            },
            4 => {
                if self.mana >= 173 && boss.poison == 0 {
                    cast = 173;
                    self.mana = self.mana - 173;
                    boss.poison = 6;
                }
            },
            5 => {
                if self.mana >= 229 && self.recharge == 0 {
                    cast = 229;
                    self.mana = self.mana - 229;
                    self.recharge = 5;
                }
            },
            _ => { cast = 0; }
        }

        (boss, cast)
    }
}


fn main() {
    let boss = Boss { hp: 51, damage: 9, poison: 0 };
    let player = Player { hp: 50, mana: 500, shield: 0, recharge: 0 };

    let mut bfs: Vec<(Boss, Player, usize)> = Vec::new();
    bfs.push((boss.clone(), player.clone(), 0));

    let mut min = usize::MAX;

    while bfs.len() > 0 {
        println!("{}, {}", bfs.len(), min);
        let (b, p,cost) = bfs.remove(0);
        for x in 1..6 {
            let mut p_tmp = p.clone();
            let (mut new_boss, cast) = p_tmp.turn(b.clone(),  x);
            let new_cost = cost + cast;

            if p_tmp.hp == 0 { continue; }
            
            if new_boss.hp == 0 {
                if new_cost < min {
                    min = new_cost;
                    continue;
                }
            }

            if new_cost < min {
                let new_player = new_boss.turn(p_tmp.clone());
                if new_boss.hp == 0 {
                    if new_cost < min {
                        min = new_cost;
                        continue;
                    }
                }

                if new_player.hp > 0 {
                    bfs.push((new_boss, new_player, new_cost));
                }
            }
        }
    }

    println!("{}", min);
}

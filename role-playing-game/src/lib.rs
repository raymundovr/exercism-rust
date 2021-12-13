// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            return Some(Player {
                health: 100,
                level: self.level,
                mana: if self.level >= 10 { Some(100) } else { None },
            });
        }

        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            self.health = if self.health >= mana_cost {
                self.health - mana_cost
            } else {
                0
            };
            return 0;
        }

        let mana = self.mana.unwrap();
        if mana < mana_cost {
            return 0;
        }

        self.mana = Some(mana - mana_cost);
        mana_cost * 2
    }
}

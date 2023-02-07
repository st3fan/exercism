use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

fn mana_for_level(level: u32) -> Option<u32> {
    return if level >= 10 { Some(100) } else { None };
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0 {
            return None;
        }

        Some(Player {
            health: 100,
            mana: mana_for_level(self.level),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana < (2 * mana_cost) {
                    0
                } else {
                    self.mana = Some(mana - min(mana_cost, mana));
                    2 * mana_cost
                }
            }
            None => {
                self.health -= min(mana_cost, self.health);
                0
            }
        }
    }
}

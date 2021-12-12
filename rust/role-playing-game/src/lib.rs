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
        match &self {
            Player {
                health: 0,
                mana: Some(..),
                level,
            } => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            Player {
                health: 0,
                mana: None,
                level,
            } => Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) if m >= mana_cost => {
                self.mana = Some(m - mana_cost);
                mana_cost * 2
            }
            Some(m) if m < mana_cost => 0,
            _ => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}

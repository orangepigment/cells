use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    str::FromStr,
};

use regex::Regex;

use crate::errors::{CellsError, Result};

#[derive(Debug, PartialEq)]
pub struct GameRules {
    inner: Rc<InnerGameRules>, // Maybe switch to Arc for thread safety
}

#[derive(Debug, PartialEq)]
struct InnerGameRules {
    born: HashSet<u8>,
    survive: HashSet<u8>,
}

impl GameRules {
    pub fn is_born(&self, alive_neighbors: u8) -> bool {
        self.inner.born.contains(&alive_neighbors)
    }

    pub fn has_survived(&self, alive_neighbors: u8) -> bool {
        self.inner.survive.contains(&alive_neighbors)
    }
}

impl FromStr for GameRules {
    type Err = CellsError;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"B(0?1?2?3?4?5?6?7?8?)/S(0?1?2?3?4?5?6?7?8?)")
            .expect("Const regex must be valid");

        let Some(caps) = re.captures(s) else {
            return Err(CellsError::GameRulesParsing(s.to_string()));
        };

        let born_str = &caps[1];
        let survive_str = &caps[2];

        let mut born: HashSet<u8> = HashSet::new();
        born_str
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .for_each(|d| {
                born.insert(d);
            });

        let mut survive: HashSet<u8> = HashSet::new();
        survive_str
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .for_each(|d| {
                survive.insert(d);
            });

        let inner = Rc::new(InnerGameRules { born, survive });
        Ok(GameRules { inner })
    }
}

#[derive(Debug)]
pub struct RulePresets {
    presets: HashMap<PresetName, Rc<InnerGameRules>>,
}

impl RulePresets {
    pub fn new() -> Self {
        RulePresets {
            presets: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: PresetName) -> GameRules {
        match name {
            PresetName::GameOfLife => self.game_of_life(),
        }
    }

    pub fn game_of_life(&mut self) -> GameRules {
        let preset = self
            .presets
            .entry(PresetName::GameOfLife)
            .or_insert_with(|| {
                let mut born: HashSet<u8> = HashSet::new();
                born.insert(3);

                let mut survive: HashSet<u8> = HashSet::new();
                survive.insert(2);
                survive.insert(3);

                Rc::new(InnerGameRules { born, survive })
            });
        GameRules {
            inner: Rc::clone(preset),
        }
    }
}

impl Default for RulePresets {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PresetName {
    GameOfLife,
}

#[cfg(test)]
mod tests {
    use std::{assert_matches, collections::HashSet};

    use crate::errors::CellsError;

    use super::*;

    // TODO: add more cases with empty B or S. Use proptest?
    #[test]
    fn parse_valid_game_rules() {
        let mut born: HashSet<u8> = HashSet::new();
        born.insert(3);

        let mut survive: HashSet<u8> = HashSet::new();
        survive.insert(2);
        survive.insert(3);

        let rules = GameRules::from_str("B3/S23").expect("B3/S23 should be parsed without errors");

        assert_eq!(rules.inner.born, born);
        assert_eq!(rules.inner.survive, survive);
    }

    #[test]
    fn parse_invalid_game_rules() {
        assert_matches!(
            GameRules::from_str("B3:S23").expect_err("B3:S23 parsing should return error"),
            CellsError::GameRulesParsing(
                s
            ) if s == "B3:S23"
        );
    }

    // Automaton tests
}

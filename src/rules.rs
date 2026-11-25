use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::{Arc, LazyLock},
};

use regex::Regex;

use crate::errors::{CellsError, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct GameRules {
    inner: Arc<InnerGameRules>, // Maybe switch to Arc for thread safety
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

static RULES_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"B(0?1?2?3?4?5?6?7?8?)/S(0?1?2?3?4?5?6?7?8?)").expect("Const regex must be valid")
});

impl FromStr for GameRules {
    type Err = CellsError;

    fn from_str(s: &str) -> Result<Self> {
        let Some(caps) = RULES_PATTERN.captures(s) else {
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

        let inner = Arc::new(InnerGameRules { born, survive });
        Ok(GameRules { inner })
    }
}

#[derive(Debug)]
pub struct RulePresets {
    presets: HashMap<PresetName, Arc<InnerGameRules>>,
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

                Arc::new(InnerGameRules { born, survive })
            });
        GameRules {
            inner: Arc::clone(preset),
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
    use proptest::prelude::*;
    use proptest::sample::subsequence;

    use super::*;

    proptest! {
        #[test]
        fn parse_valid_game_rules(
            born in subsequence((0..=8).collect::<Vec<u8>>(), 0..=9),
            survive in subsequence((0..=8).collect::<Vec<u8>>(), 0..=9)
        ) {
            let mut born_set: HashSet<u8> = HashSet::new();
            born.iter().for_each(|b| {born_set.insert(*b);});

            let mut survive_set: HashSet<u8> = HashSet::new();
            survive.iter().for_each(|s| {survive_set.insert(*s);});

            let raw_rules = format!("B{}/S{}",
                born.iter().map(|b| char::from_digit(*b as u32, 10).unwrap()).collect::<String>(),
                survive.iter().map(|s| char::from_digit(*s as u32, 10).unwrap()).collect::<String>()
            );
            let rules = GameRules::from_str(&raw_rules).unwrap_or_else(|_|
                panic!("Rules string '{raw_rules}' should be parsed without errors")
            );

            assert_eq!(rules.inner.born, born_set);
            assert_eq!(rules.inner.survive, survive_set);
        }
    }

    proptest! {
        #[test]
        fn parse_invalid_game_rules(raw_rules in "\\PC*".prop_filter("Values must not be a valid rule string", |s| !RULES_PATTERN.is_match(s))) {
            assert_matches!(
                GameRules::from_str(&raw_rules).map(|_|
                    format!("Rules string '{raw_rules}' parsing should return error")
                ).unwrap_err(),
                CellsError::GameRulesParsing(
                    s
                ) if s == raw_rules
            );
        }
    }

    // TODO: add tests for presets
}

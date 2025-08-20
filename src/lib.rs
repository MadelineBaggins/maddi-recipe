// SPDX-FileCopyrightText: 2025 Madeline Baggins <madeline@baggins.family>
//
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use std::{borrow::Cow, fmt::Display};

trait SplitTwice<'a> {
    fn split_twice(self, delim: &'a str) -> Option<(&'a str, &'a str, &'a str)>;
}

impl<'a> SplitTwice<'a> for &'a str {
    fn split_twice(self: &'a str, delim: &'a str) -> Option<(&'a str, &'a str, &'a str)> {
        self.split_once(delim)
            .and_then(|(a, b)| b.split_once(delim).map(|(b, c)| (a, b, c)))
    }
}

#[derive(Debug, Clone)]
pub struct Recipe<'a> {
    pub preface: Cow<'a, str>,
    pub ingredients: Vec<Ingredient<'a>>,
    pub instructions: Cow<'a, str>,
}

impl<'a> Recipe<'a> {
    pub fn into_static(self) -> Recipe<'static> {
        let Self {
            preface,
            ingredients,
            instructions,
        } = self;
        Recipe {
            preface: preface.to_string().into(),
            ingredients: ingredients.into_iter().map(|i| i.into_static()).collect(),
            instructions: instructions.to_string().into(),
        }
    }
}

impl Display for Recipe<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.preface)?;
        for ingredient in &self.ingredients {
            write!(f, "{ingredient}")?;
        }
        write!(f, "{}", self.instructions)
    }
}

#[derive(Debug, Clone)]
pub struct Ingredient<'a> {
    pub indent: Cow<'a, str>,
    pub quantity: Quantity,
    pub name: Cow<'a, str>,
}

impl Display for Ingredient<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}- ", self.indent)?;
        match &self.quantity {
            Quantity::Simple(q) => write!(f, "{q} ")?,
            Quantity::Volume(v) => write!(f, "{v} ")?,
            _ => (),
        };
        write!(f, "{}", self.name)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Quantity {
    None,
    Simple(f32),
    Volume(Volume),
}

#[derive(Debug, Clone)]
pub struct Volume {
    quarter_teaspoons: f32,
}

impl Volume {
    pub fn quarter_teaspoons(&self) -> f32 {
        self.quarter_teaspoons
    }
    pub fn scale(&self, factor: f32) -> Self {
        Volume {
            quarter_teaspoons: self.quarter_teaspoons * factor,
        }
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use quarter_teaspoons::*;
        let mut qtr_tsps = self.quarter_teaspoons;
        let mut out = String::new();
        // Take out as many cups as you can.
        let mut plural = false;
        let cups = qtr_tsps.div_euclid(CUP);
        qtr_tsps = qtr_tsps.rem_euclid(CUP);
        if cups > 0.0 {
            out.push_str(&cups.to_string());
            out.push(' ');
        }
        // Check if 3/4 cup remains
        if qtr_tsps >= THREE_QUARTER_CUP {
            if !out.is_empty() {
                out.push_str("+ ");
                plural = true;
            }
            out.push_str("3/4 ");
            qtr_tsps -= THREE_QUARTER_CUP;
        }
        // Check if 2/3 Cup remains
        if qtr_tsps >= TWO_THIRDS_CUP {
            if !out.is_empty() {
                out.push_str("+ ");
                plural = true;
            }
            out.push_str("2/3 ");
            qtr_tsps -= TWO_THIRDS_CUP;
        }
        // Check if 1/2 Cup remains
        if qtr_tsps >= HALF_CUP {
            if !out.is_empty() {
                out.push_str("+ ");
                plural = true;
            }
            out.push_str("1/2 ");
            qtr_tsps -= HALF_CUP;
        }
        // Check if 1/3 Cup remains
        if qtr_tsps >= THIRD_CUP {
            if !out.is_empty() {
                out.push_str("+ ");
                plural = true;
            }
            out.push_str("1/3 ");
            qtr_tsps -= THIRD_CUP;
        }
        // Check if 1/4 Cup remains
        if qtr_tsps >= QUARTER_CUP {
            if !out.is_empty() {
                out.push_str("+ ");
                plural = true;
            }
            out.push_str("1/4 ");
            qtr_tsps -= QUARTER_CUP;
        }
        // Add 'cups' or 'cup'
        if cups > 1.0 || plural {
            out.push_str("cups");
        } else if !out.is_empty() {
            out.push_str("cup");
        }

        // Adding tablespoons
        let mut has_tablespoons = false;
        let mut plural = false;
        let tablespoons = qtr_tsps.div_euclid(TABLESPOON);
        qtr_tsps = qtr_tsps.rem_euclid(TABLESPOON);
        if tablespoons > 0.0 {
            has_tablespoons = true;
            if !out.is_empty() {
                out.push_str("+ ");
            }
            out.push_str(&format!("{tablespoons} "));
        }
        // As two teaspoons is more than half a tablespoon, we only
        // do this one if we have less than two teaspoons
        if (HALF_TABLESPOON..2.0 * TEASPOON).contains(&qtr_tsps) {
            if !out.is_empty() {
                out.push_str("+ ");
            }
            plural = has_tablespoons;
            has_tablespoons = true;
            out.push_str("1/2 ");
            qtr_tsps -= HALF_TABLESPOON;
        }
        if tablespoons > 1.0 || plural {
            out.push_str("tbsps");
        } else if has_tablespoons {
            out.push_str("tbsp")
        }

        // Adding teaspoons
        let mut has_teaspoons = false;
        let mut plural = false;
        let teaspoons = qtr_tsps.div_euclid(TEASPOON);
        qtr_tsps = qtr_tsps.rem_euclid(TEASPOON);
        if teaspoons > 0.0 {
            has_teaspoons = true;
            if !out.is_empty() {
                out.push_str("+ ");
            }
            out.push_str(&format!("{teaspoons} "));
        }
        if qtr_tsps >= HALF_TEASPOON {
            plural = has_teaspoons;
            has_teaspoons = true;
            if !out.is_empty() {
                out.push_str("+ ");
            }
            out.push_str("1/2 ");
            qtr_tsps -= HALF_TEASPOON;
        }
        if qtr_tsps >= QUARTER_TEASPOON {
            plural = has_teaspoons;
            has_teaspoons = true;
            if !out.is_empty() {
                out.push_str("+ ");
            }
            out.push_str("1/4 ");
            qtr_tsps -= QUARTER_TEASPOON;
        }
        if qtr_tsps > 0.0 {
            plural = has_teaspoons;
            has_teaspoons = true;
            if !out.is_empty() {
                out.push_str("+ ");
            }
            let tsps = qtr_tsps / 4.0;
            match tsps {
                0.0625 => out.push_str("1/16 "),
                0.125 => out.push_str("1/8 "),
                tsps => out.push_str(&format!("{tsps} ")),
            }
        }
        if teaspoons > 1.0 || plural {
            out.push_str("tsps");
        } else if has_teaspoons {
            out.push_str("tsp")
        }
        // TODO

        // Adding teaspoons
        // - Check if what's left is greater or equal to two teaspoons
        // - Do the rest
        write!(f, "{out}")
    }
}

mod quarter_teaspoons {
    pub const CUP: f32 = 16.0 * 3.0 * 4.0;
    pub const THREE_QUARTER_CUP: f32 = 3.0 / 4.0 * CUP;
    pub const TWO_THIRDS_CUP: f32 = 2.0 / 3.0 * CUP;
    pub const HALF_CUP: f32 = 0.5 * CUP;
    pub const THIRD_CUP: f32 = 1.0 / 3.0 * CUP;
    pub const QUARTER_CUP: f32 = 1.0 / 4.0 * CUP;
    pub const TABLESPOON: f32 = 3.0 * 4.0;
    pub const HALF_TABLESPOON: f32 = 0.5 * TABLESPOON;
    pub const TEASPOON: f32 = 4.0;
    pub const HALF_TEASPOON: f32 = 2.0;
    pub const QUARTER_TEASPOON: f32 = 1.0;
}

impl Volume {
    fn parse(amount: &str, unit: &str) -> Option<Self> {
        let amount = parse_f32(amount).ok()?;
        let unit_quarter_teaspoons: f32 = match unit.to_lowercase().as_str() {
            "cups" | "cup" => 16.0 * 3.0 * 4.0,
            "tablespoon" | "tablespoons" | "tb" | "tbs" | "tbsp" | "tbsps" => 3.0 * 4.0,
            "teaspoon" | "teaspoons" | "tsp" | "tsps" => 4.0,
            _ => return None,
        };
        Some(Self {
            quarter_teaspoons: amount * unit_quarter_teaspoons,
        })
    }
}

impl<'a> Recipe<'a> {
    pub fn scale(&self, factor: f32) -> Self {
        Recipe {
            preface: self.preface.clone(),
            ingredients: self.ingredients.iter().map(|i| i.scale(factor)).collect(),
            instructions: self.instructions.clone(),
        }
    }
    pub fn parse(src: &'a str) -> Self {
        // Find where the ingredients start
        const INGREDIENTS: &str = "\n## Ingredients\n\n";
        let Some(mut ingredients_start) = src.find(INGREDIENTS) else {
            return Recipe {
                preface: Cow::Borrowed(src),
                ingredients: vec![],
                instructions: Cow::Borrowed(""),
            };
        };
        ingredients_start += INGREDIENTS.len();
        // Seperate the preface, ingredients, and instructions
        let (preface, src) = src.split_at(ingredients_start);
        let (ingredients, instructions) = match src.find("\n##") {
            Some(ingredients_end) => src.split_at(ingredients_end),
            None => (src, ""),
        };
        // Parse the ingredients
        let ingredients = Ingredients(ingredients).map(Ingredient::parse).collect();

        // Return the recipe
        Recipe {
            preface: preface.into(),
            ingredients,
            instructions: instructions.into(),
        }
    }
}

fn parse_f32(num: &str) -> Result<f32, std::num::ParseFloatError> {
    if let Some((a, b)) = num.split_once("/") {
        Ok(a.parse::<f32>()? / b.parse::<f32>()?)
    } else {
        num.parse::<f32>()
    }
}

impl<'a> Ingredient<'a> {
    fn into_static(self) -> Ingredient<'static> {
        let Self {
            indent,
            quantity,
            name,
        } = self;
        Ingredient {
            indent: indent.to_string().into(),
            quantity,
            name: name.to_string().into(),
        }
    }
    fn scale(&self, factor: f32) -> Self {
        let quantity = match &self.quantity {
            Quantity::None => Quantity::None,
            Quantity::Simple(q) => Quantity::Simple(q * factor),
            Quantity::Volume(volume) => Quantity::Volume(volume.scale(factor)),
        };
        Self {
            indent: self.indent.clone(),
            quantity,
            name: self.name.clone(),
        }
    }
    fn parse(src: &'a str) -> Self {
        let (indent, tail) = src
            .split_once("- ")
            .expect("Attempted to parse a non-ingredient string.");
        let (quantity, name) = 'parse_quantity: {
            // Try to parse as a volume
            if let Some((amount, unit, name)) = tail.split_twice(" ")
                && let Some(volume) = Volume::parse(amount, unit)
            {
                break 'parse_quantity (Quantity::Volume(volume), name);
            };
            // Try to parse as a simple
            if let Some((amount, name)) = tail.split_once(" ")
                && let Ok(simple) = parse_f32(amount)
            {
                break 'parse_quantity (Quantity::Simple(simple), name);
            }
            // Resort to a none
            (Quantity::None, tail)
        };
        Self {
            indent: indent.into(),
            quantity,
            name: name.into(),
        }
    }
}

struct Ingredients<'a>(&'a str);

impl<'a> Iterator for Ingredients<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Store the current tail
        let src = self.0;
        // Skip past the start of the md item
        let (_, tail) = src.split_once("- ")?;
        // Find the start of the next item
        for line in tail.split("\n") {
            if line.trim_start().starts_with("-") {
                let end = line.as_ptr() as usize;
                let len = end - src.as_ptr() as usize;
                let (next, src) = src.split_at(len);
                self.0 = src;
                return Some(next);
            }
        }
        // If we can't, return everything
        self.0 = "";
        Some(src)
    }
}

#[test]
fn pizza() {
    let pizza_src = include_str!("tests/pizza.md"); // Lol 'pizza_src'
    let recipe = Recipe::parse(pizza_src);
    let scaled = recipe.scale(0.5);
    println!("{scaled}");
    assert_eq!(pizza_src, format!("{recipe}"));
}

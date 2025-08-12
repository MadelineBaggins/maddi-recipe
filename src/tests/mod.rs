// SPDX-FileCopyrightText: 2025 Madeline Baggins <madeline@baggins.family>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::Recipe;

#[test]
fn pizza() {
    let pizza_src = include_str!("pizza.md"); // Lol 'pizza_src'
    let recipe = Recipe::parse(pizza_src);
    let scaled = recipe.scale(0.5);
    println!("{scaled}");
    assert_eq!(pizza_src, format!("{recipe}"));
}

use crate::config::{BASE_COLOR, HIGHLIGHT_COLOR};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub money: i32,
    pub credit: i32,
    pub change_input: String,
}

impl PlayerState {
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            money: 20,
            credit: 0,
            change_input: String::new(),
        }
    }

    pub fn apply_money(&mut self) {
        if let Ok(delta) = self.change_input.trim().parse::<i32>() {
            self.money += delta;
            self.change_input.clear();
        }
    }

    pub fn add_credit(&mut self) {
        self.money += 10;
        self.credit += 1;
    }

    pub fn subtract_credit(&mut self) {
        if self.money >= 10 && self.credit > 0 {
            self.money -= 10;
            self.credit -= 1;
        }
    }

    pub fn pay_credit_interest(&mut self) {
        let interest = self.credit;

        while self.money < interest {
            self.add_credit();
        }

        self.money -= interest;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CellState {
    pub label: String,
    pub is_highlight: bool,
    pub color: String,
    pub is_arrow: bool,
}

impl CellState {
    pub fn paint(&mut self, selected_color: &str) {
        if self.color == selected_color {
            self.color = if self.is_highlight {
                HIGHLIGHT_COLOR.to_string()
            } else {
                BASE_COLOR.to_string()
            };
        } else {
            self.color = selected_color.to_string();
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarketState {
    pub title: String,
    pub bg_color: String,
    pub prices_cells: Vec<CellState>,
    pub holdings_cells: Vec<CellState>,
    pub shorts_cells: Vec<CellState>,
}

#[derive(Clone)]
pub struct HistorySnapshot {
    pub player1: PlayerState,
    pub player2: PlayerState,
    pub player3: PlayerState,
    pub player4: PlayerState,
    pub markets: Vec<MarketState>,
}

use dioxus::prelude::*;
use crate::config::{PAINT_COLORS, PURPLE_COLOR};
use crate::market::{build_markets, shift_prices_cells_left, shift_prices_cells_right};
use crate::state::{MarketState, PlayerState};
use serde::{Deserialize, Serialize};


const STORAGE_KEY: &str = "hulipoliya_game_state";

#[derive(Serialize, Deserialize, Clone)]
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
    player3: PlayerState,
    player4: PlayerState,
    player_colors: Vec<String>,
    markets: Vec<MarketState>,
}

fn save_to_localstorage(
    player1: &PlayerState,
    player2: &PlayerState,
    player3: &PlayerState,
    player4: &PlayerState,
    player_colors: &[String],
    markets: &[MarketState],
) {
    let state = GameState {
        player1: player1.clone(),
        player2: player2.clone(),
        player3: player3.clone(),
        player4: player4.clone(),
        player_colors: player_colors.to_vec(),
        markets: markets.to_vec(),
    };
    
    if let Ok(json) = serde_json::to_string(&state) {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        let _ = storage.set_item(STORAGE_KEY, &json);
    }
}

fn load_from_localstorage() -> Option<GameState> {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    
    if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
        if let Ok(state) = serde_json::from_str(&json) {
            return Some(state);
        }
    }
    None
}

fn clear_localstorage() {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let _ = storage.remove_item(STORAGE_KEY);
}

fn reset_game(
    mut player1: Signal<PlayerState>,
    mut player2: Signal<PlayerState>,
    mut player3: Signal<PlayerState>,
    mut player4: Signal<PlayerState>,
    mut markets: Signal<Vec<MarketState>>,
) {
    // Reset player states (keep names and colors, reset money/credit/input)
    player1.with_mut(|p| {
        p.money = 20;
        p.credit = 0;
        p.change_input.clear();
    });
    player2.with_mut(|p| {
        p.money = 20;
        p.credit = 0;
        p.change_input.clear();
    });
    player3.with_mut(|p| {
        p.money = 20;
        p.credit = 0;
        p.change_input.clear();
    });
    player4.with_mut(|p| {
        p.money = 20;
        p.credit = 0;
        p.change_input.clear();
    });

    // Reset markets to initial state
    markets.set(build_markets());
}

#[component]
pub fn App() -> Element {
    // Load from localStorage or use defaults
    let initial_state = load_from_localstorage();
    
    let player1 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player1.clone()).unwrap_or_else(|| PlayerState::with_name("Player 1"))
    });
    let player2 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player2.clone()).unwrap_or_else(|| PlayerState::with_name("Player 2"))
    });
    let player3 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player3.clone()).unwrap_or_else(|| PlayerState::with_name("Player 3"))
    });
    let player4 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player4.clone()).unwrap_or_else(|| PlayerState::with_name("Player 4"))
    });
    let selected_color = use_signal(|| PAINT_COLORS[0].1.to_string());
    let player_colors = use_signal(|| {
        initial_state.as_ref().map(|s| s.player_colors.clone())
            .unwrap_or_else(|| PAINT_COLORS.iter().map(|(_, c)| c.to_string()).collect::<Vec<_>>())
    });
    let drag_source: Signal<Option<usize>> = use_signal(|| None);
    let mut markets = use_signal(|| {
        initial_state.map(|s| s.markets).unwrap_or_else(build_markets)
    });
    let mut show_modal = use_signal(|| false);
    
    // Effect to save state whenever it changes
    use_effect(move || {
        save_to_localstorage(
            &player1(),
            &player2(),
            &player3(),
            &player4(),
            &player_colors(),
            &markets(),
        );
    });

    rsx! {
        style { {include_str!("../assets/main.css")} }

        if show_modal() {
            ConfirmModal {
                show_modal,
                on_confirm: move |_| {
                    reset_game(player1, player2, player3, player4, markets);
                    clear_localstorage();
                    show_modal.set(false);
                },
            }
        }

        div { class: "app",
            div { class: "players",
                PlayerPanel { state: player1, player_idx: 0, player_colors, drag_source, selected_color, markets }
                PlayerPanel { state: player2, player_idx: 1, player_colors, drag_source, selected_color, markets }
                PlayerPanel { state: player3, player_idx: 2, player_colors, drag_source, selected_color, markets }
                PlayerPanel { state: player4, player_idx: 3, player_colors, drag_source, selected_color, markets }
                GameControls { show_modal }
            }

            hr { class: "divider" }

            for market_idx in 0..markets().len() {
                div { class: "market-row",
                    div { class: "left-scale-wrapper",
                        button {
                            class: "shift-btn",
                            onclick: move |_| {
                                markets.with_mut(|m| {
                                    shift_prices_cells_left(&mut m[market_idx].prices_cells);
                                });
                            },
                            "◀"
                        }
                        div { class: "left-scale",
                            for cell_idx in 0..markets()[market_idx].prices_cells.len() {
                                button {
                                    class: "price-cell",
                                    style: "background-color: {markets()[market_idx].prices_cells[cell_idx].color};",
                                    onclick: move |_| {
                                        let current = selected_color();
                                        markets.with_mut(|m| {
                                            m[market_idx].prices_cells[cell_idx].paint(&current);
                                        });
                                    },
                                    "{markets()[market_idx].prices_cells[cell_idx].label}"
                                }
                            }
                        }
                        button {
                            class: "shift-btn",
                            onclick: move |_| {
                                markets.with_mut(|m| {
                                    shift_prices_cells_right(&mut m[market_idx].prices_cells);
                                });
                            },
                            "▶"
                        }
                    }

                    div {
                        class: "market-title",
                        style: "background-color: {markets()[market_idx].bg_color};",
                        if markets()[market_idx].title == "Country Stocks" {
                            "Country"
                            br {}
                            "Stocks"
                        } else {
                            "{markets()[market_idx].title}"
                        }
                    }

                    div { class: "arrow-rows",
                        div { class: "arrow-row",
                            for cell_idx in 0..markets()[market_idx].holdings_cells.len() {
                                button {
                                    class: if markets()[market_idx].holdings_cells[cell_idx].is_arrow { "cell arrow-gap" } else { "cell" },
                                    style: "background-color: {markets()[market_idx].holdings_cells[cell_idx].color};",
                                    onclick: move |_| {
                                        let current = selected_color();
                                        markets.with_mut(|m| {
                                            m[market_idx].holdings_cells[cell_idx].paint(&current);
                                        });
                                    },
                                    "{markets()[market_idx].holdings_cells[cell_idx].label}"
                                }
                            }
                        }

                        div { class: "arrow-row",
                            for cell_idx in 0..markets()[market_idx].shorts_cells.len() {
                                button {
                                    class: if markets()[market_idx].shorts_cells[cell_idx].is_arrow { "cell arrow-gap" } else { "cell" },
                                    style: "background-color: {markets()[market_idx].shorts_cells[cell_idx].color};",
                                    onclick: move |_| {
                                        let current = selected_color();
                                        markets.with_mut(|m| {
                                            m[market_idx].shorts_cells[cell_idx].paint(&current);
                                        });
                                    },
                                    "{markets()[market_idx].shorts_cells[cell_idx].label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GameControls(show_modal: Signal<bool>) -> Element {
    rsx! {
        div { class: "game-controls",
            button {
                class: "new-game-btn",
                onclick: move |_| show_modal.set(true),
                "Start new game"
            }
        }
    }
}

#[component]
fn ConfirmModal(show_modal: Signal<bool>, on_confirm: EventHandler<()>) -> Element {
    rsx! {
        div { class: "modal-overlay",
            onclick: move |_| show_modal.set(false),
            div { class: "modal-content",
                onclick: move |evt| evt.stop_propagation(),
                h3 { "Start New Game" }
                p { "Are you sure you want to start a new game? All game progress will be reset, but player names and colors will be preserved." }
                div { class: "modal-buttons",
                    button {
                        class: "modal-btn cancel",
                        onclick: move |_| show_modal.set(false),
                        "Cancel"
                    }
                    button {
                        class: "modal-btn confirm",
                        onclick: move |_| on_confirm.call(()),
                        "Confirm"
                    }
                }
            }
        }
    }
}

#[component]
fn PlayerPanel(
    state: Signal<PlayerState>,
    player_idx: usize,
    mut player_colors: Signal<Vec<String>>,
    mut drag_source: Signal<Option<usize>>,
    mut selected_color: Signal<String>,
    markets: Signal<Vec<MarketState>>,
) -> Element {
    let capital = {
        let s = state();
        let color = player_colors()[player_idx].clone();
        let mks = markets();
        let base = s.money - 10 * s.credit;
        let stock_sum: i32 = mks.iter().map(|market| {
            let mut purple_indices: Vec<i32> = market.prices_cells.iter()
                .filter(|c| c.color == PURPLE_COLOR)
                .filter_map(|c| c.label.parse::<i32>().ok())
                .collect();
            purple_indices.sort();
            let sell_price = purple_indices.first().copied().unwrap_or(0);
            let buy_price = purple_indices.last().copied().unwrap_or(0);
            let held = market.holdings_cells.iter().filter(|c| c.color == color).count() as i32;
            let sold = market.shorts_cells.iter().filter(|c| c.color == color).count() as i32;
            held * sell_price - sold * buy_price
        }).sum();
        base + stock_sum
    };

    rsx! {
        div { class: "player-panel",
            input {
                class: "player-title-input",
                value: "{state().name}",
                oninput: move |evt| {
                    let value = evt.value();
                    state.with_mut(|s| s.name = value);
                }
            }

            div { class: "player-row",
                span { "Credit:" }
                span { "{state().credit}" }
                button {
                    class: "control-btn",
                    onclick: move |_| state.with_mut(PlayerState::subtract_credit),
                    "-"
                }
                button {
                    class: "control-btn",
                    onclick: move |_| state.with_mut(PlayerState::add_credit),
                    "+"
                }
            }

            div { class: "player-row",
                div { class: "half-col",
                    span { "Money:" }
                    span { "{state().money}" }
                }
                div { class: "half-col",
                    span { "Capital:" }
                    span { "{capital}" }
                }
            }

            div { class: "player-row",
                input {
                    class: "money-input",
                    placeholder: "+ / - money",
                    value: "{state().change_input}",
                    oninput: move |evt| {
                        let value = evt.value();
                        state.with_mut(|s| s.change_input = value);
                    }
                }
                button {
                    class: "control-btn apply-btn",
                    onclick: move |_| state.with_mut(PlayerState::apply_money),
                    "Apply"
                }
            }

            button {
                class: if selected_color() == player_colors()[player_idx] { "color-pick-btn selected" } else { "color-pick-btn" },
                style: "background-color: {player_colors()[player_idx]}; cursor: grab;",
                draggable: true,
                onclick: move |_| selected_color.set(player_colors()[player_idx].clone()),
                ondragstart: move |_| drag_source.set(Some(player_idx)),
                ondragover: move |evt| evt.prevent_default(),
                ondrop: move |evt| {
                    evt.prevent_default();
                    if let Some(src) = drag_source() {
                        if src != player_idx {
                            player_colors.with_mut(|colors| colors.swap(src, player_idx));
                        }
                        drag_source.set(None);
                    }
                },
            }
        }
    }
}
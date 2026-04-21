use dioxus::prelude::*;
use crate::config::{PAINT_COLORS, PURPLE_COLOR};
use crate::history::History;
use crate::market::{
    build_markets,
    paint_holdings_or_clear_shorts,
    paint_shorts_or_clear_holdings,
    shift_prices_cells_left,
    shift_prices_cells_right,
};
use crate::state::{HistorySnapshot, MarketState, PlayerState};
use serde::{Deserialize, Serialize};


const STORAGE_KEY: &str = "hulipoliya_game_state";

#[derive(Serialize, Deserialize, Clone)]
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
    player3: PlayerState,
    player4: PlayerState,
    fp_owner: usize,
    fp_value: usize,
    player_colors: Vec<String>,
    markets: Vec<MarketState>,
}

fn save_to_localstorage(
    player1: &PlayerState,
    player2: &PlayerState,
    player3: &PlayerState,
    player4: &PlayerState,
    fp_owner: usize,
    fp_value: usize,
    player_colors: &[String],
    markets: &[MarketState],
) {
    let state = GameState {
        player1: player1.clone(),
        player2: player2.clone(),
        player3: player3.clone(),
        player4: player4.clone(),
        fp_owner,
        fp_value,
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
    mut fp_owner: Signal<usize>,
    mut fp_value: Signal<usize>,
    mut markets: Signal<Vec<MarketState>>,
    mut history: Signal<History>,
) {
    player1.with_mut(|p| { p.money = 20; p.credit = 0; p.change_input.clear(); });
    player2.with_mut(|p| { p.money = 20; p.credit = 0; p.change_input.clear(); });
    player3.with_mut(|p| { p.money = 20; p.credit = 0; p.change_input.clear(); });
    player4.with_mut(|p| { p.money = 20; p.credit = 0; p.change_input.clear(); });
    fp_owner.set(0);
    fp_value.set(1);
    markets.set(build_markets());
    history.with_mut(|h| { h.undo_stack.clear(); h.redo_stack.clear(); });
}

fn pay_credit_interest_for_all(
    mut player1: Signal<PlayerState>,
    mut player2: Signal<PlayerState>,
    mut player3: Signal<PlayerState>,
    mut player4: Signal<PlayerState>,
) {
    player1.with_mut(PlayerState::pay_credit_interest);
    player2.with_mut(PlayerState::pay_credit_interest);
    player3.with_mut(PlayerState::pay_credit_interest);
    player4.with_mut(PlayerState::pay_credit_interest);
}

fn count_player_shorts(markets: &[MarketState], color: &str) -> i32 {
    markets
        .iter()
        .map(|market| market.shorts_cells.iter().filter(|cell| cell.color == color).count() as i32)
        .sum()
}

fn pay_shorts_fee_for_all(
    mut player1: Signal<PlayerState>,
    mut player2: Signal<PlayerState>,
    mut player3: Signal<PlayerState>,
    mut player4: Signal<PlayerState>,
    player_colors: &[String],
    markets: &[MarketState],
) {
    let player1_fee = (count_player_shorts(markets, &player_colors[0]) + 1) / 2;
    let player2_fee = (count_player_shorts(markets, &player_colors[1]) + 1) / 2;
    let player3_fee = (count_player_shorts(markets, &player_colors[2]) + 1) / 2;
    let player4_fee = (count_player_shorts(markets, &player_colors[3]) + 1) / 2;

    player1.with_mut(|player| player.money -= player1_fee);
    player2.with_mut(|player| player.money -= player2_fee);
    player3.with_mut(|player| player.money -= player3_fee);
    player4.with_mut(|player| player.money -= player4_fee);
}

fn give_salary_for_all(
    mut player1: Signal<PlayerState>,
    mut player2: Signal<PlayerState>,
    mut player3: Signal<PlayerState>,
    mut player4: Signal<PlayerState>,
) {
    player1.with_mut(|player| player.money += 5);
    player2.with_mut(|player| player.money += 5);
    player3.with_mut(|player| player.money += 5);
    player4.with_mut(|player| player.money += 5);
}

fn apply_money_delta_for_color(
    mut player1: Signal<PlayerState>,
    mut player2: Signal<PlayerState>,
    mut player3: Signal<PlayerState>,
    mut player4: Signal<PlayerState>,
    player_colors: &[String],
    selected_color: &str,
    money_delta: i32,
) {
    if money_delta == 0 {
        return;
    }

    match player_colors.iter().position(|color| color == selected_color) {
        Some(0) => player1.with_mut(|player| player.money += money_delta),
        Some(1) => player2.with_mut(|player| player.money += money_delta),
        Some(2) => player3.with_mut(|player| player.money += money_delta),
        Some(3) => player4.with_mut(|player| player.money += money_delta),
        _ => {}
    }
}

fn position_hover_title(cell: &crate::state::CellState, action: &str) -> String {
    match (cell.is_painted(), cell.remembered_price) {
        (true, Some(price)) => format!("{action} at {price}"),
        _ => String::new(),
    }
}

#[component]
pub fn App() -> Element {
    // Load from localStorage or use defaults
    let initial_state = load_from_localstorage();
    
    let mut player1 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player1.clone()).unwrap_or_else(|| PlayerState::with_name("Player 1"))
    });
    let mut player2 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player2.clone()).unwrap_or_else(|| PlayerState::with_name("Player 2"))
    });
    let mut player3 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player3.clone()).unwrap_or_else(|| PlayerState::with_name("Player 3"))
    });
    let mut player4 = use_signal(|| {
        initial_state.as_ref().map(|s| s.player4.clone()).unwrap_or_else(|| PlayerState::with_name("Player 4"))
    });
    let mut fp_owner = use_signal(|| initial_state.as_ref().map(|s| s.fp_owner).unwrap_or(0));
    let mut fp_value = use_signal(|| initial_state.as_ref().map(|s| s.fp_value).unwrap_or(1));
    let selected_color = use_signal(|| PAINT_COLORS[0].1.to_string());
    let player_colors = use_signal(|| {
        initial_state.as_ref().map(|s| s.player_colors.clone())
            .unwrap_or_else(|| PAINT_COLORS.iter().map(|(_, c)| c.to_string()).collect::<Vec<_>>())
    });
    let drag_source: Signal<Option<usize>> = use_signal(|| None);
    let fp_drag_source: Signal<bool> = use_signal(|| false);
    let mut markets = use_signal(|| {
        initial_state.map(|s| s.markets).unwrap_or_else(build_markets)
    });
    let mut show_modal = use_signal(|| false);
    let mut history = use_signal(|| History::new());

    let make_snapshot = move || HistorySnapshot {
        player1: player1(),
        player2: player2(),
        player3: player3(),
        player4: player4(),
        fp_owner: fp_owner(),
        fp_value: fp_value(),
        markets: markets(),
    };

    let mut push_history = move || {
        history.with_mut(|h| h.push(make_snapshot()));
    };

    let mut restore_snapshot = move |s: HistorySnapshot| {
        player1.set(s.player1);
        player2.set(s.player2);
        player3.set(s.player3);
        player4.set(s.player4);
        fp_owner.set(s.fp_owner);
        fp_value.set(s.fp_value);
        markets.set(s.markets);
    };

    // Effect to save state whenever it changes
    use_effect(move || {
        save_to_localstorage(
            &player1(),
            &player2(),
            &player3(),
            &player4(),
            fp_owner(),
            fp_value(),
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
                    reset_game(player1, player2, player3, player4, fp_owner, fp_value, markets, history);
                    clear_localstorage();
                    show_modal.set(false);
                },
            }
        }

        div { class: "app",
            div { class: "players",
                PlayerPanel { state: player1, player_idx: 0, player_colors, drag_source, fp_owner, fp_value, fp_drag_source, selected_color, markets, on_push_history: move |_| push_history() }
                PlayerPanel { state: player2, player_idx: 1, player_colors, drag_source, fp_owner, fp_value, fp_drag_source, selected_color, markets, on_push_history: move |_| push_history() }
                PlayerPanel { state: player3, player_idx: 2, player_colors, drag_source, fp_owner, fp_value, fp_drag_source, selected_color, markets, on_push_history: move |_| push_history() }
                PlayerPanel { state: player4, player_idx: 3, player_colors, drag_source, fp_owner, fp_value, fp_drag_source, selected_color, markets, on_push_history: move |_| push_history() }
                GameControls {
                    show_modal,
                    history,
                    player1,
                    player2,
                    player3,
                    player4,
                    player_colors,
                    markets,
                    on_pay_percents: move |_| {
                        push_history();
                        pay_credit_interest_for_all(player1, player2, player3, player4);
                    },
                    on_pay_shorts_fee: move |_| {
                        push_history();
                        let colors = player_colors();
                        let current_markets = markets();
                        pay_shorts_fee_for_all(
                            player1,
                            player2,
                            player3,
                            player4,
                            &colors,
                            &current_markets,
                        );
                    },
                    on_give_salary: move |_| {
                        push_history();
                        give_salary_for_all(player1, player2, player3, player4);
                    },
                    on_undo: move |_| {
                        let snap = history.with_mut(|h| h.undo(make_snapshot()));
                        if let Some(s) = snap { restore_snapshot(s); }
                    },
                    on_redo: move |_| {
                        let snap = history.with_mut(|h| h.redo(make_snapshot()));
                        if let Some(s) = snap { restore_snapshot(s); }
                    },
                }
            }

            hr { class: "divider" }

            for market_idx in 0..markets().len() {
                div { class: "market-row",
                    div { class: "left-scale-wrapper",
                        button {
                            class: "shift-btn",
                            onclick: move |_| {
                                push_history();
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
                                push_history();
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
                                div { class: "position-cell-wrap",
                                    button {
                                        class: if markets()[market_idx].holdings_cells[cell_idx].is_arrow { "cell arrow-gap" } else { "cell" },
                                        style: "background-color: {markets()[market_idx].holdings_cells[cell_idx].color};",
                                        aria_label: "{position_hover_title(&markets()[market_idx].holdings_cells[cell_idx], \"Bought\")}",
                                        onclick: move |_| {
                                            let snapshot = make_snapshot();
                                            let current = selected_color();
                                            let change = markets.with_mut(|m| {
                                                paint_holdings_or_clear_shorts(
                                                    &mut m[market_idx],
                                                    cell_idx,
                                                    &current,
                                                )
                                            });
                                            if change.changed {
                                                let colors = player_colors();
                                                apply_money_delta_for_color(
                                                    player1,
                                                    player2,
                                                    player3,
                                                    player4,
                                                    &colors,
                                                    &current,
                                                    change.money_delta,
                                                );
                                                history.with_mut(|h| h.push(snapshot));
                                            }
                                        },
                                        "{markets()[market_idx].holdings_cells[cell_idx].label}"
                                    }
                                    if markets()[market_idx].holdings_cells[cell_idx].is_painted() {
                                        div { class: "position-tooltip",
                                            span { class: "position-tooltip-label", "Bought" }
                                            span { class: "position-tooltip-price", "{markets()[market_idx].holdings_cells[cell_idx].remembered_price.unwrap_or_default()}" }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "arrow-row",
                            for cell_idx in 0..markets()[market_idx].shorts_cells.len() {
                                div { class: "position-cell-wrap",
                                    button {
                                        class: if markets()[market_idx].shorts_cells[cell_idx].is_arrow { "cell arrow-gap" } else { "cell" },
                                        style: "background-color: {markets()[market_idx].shorts_cells[cell_idx].color};",
                                        aria_label: "{position_hover_title(&markets()[market_idx].shorts_cells[cell_idx], \"Sold\")}",
                                        onclick: move |_| {
                                            let snapshot = make_snapshot();
                                            let current = selected_color();
                                            let change = markets.with_mut(|m| {
                                                paint_shorts_or_clear_holdings(
                                                    &mut m[market_idx],
                                                    cell_idx,
                                                    &current,
                                                )
                                            });
                                            if change.changed {
                                                let colors = player_colors();
                                                apply_money_delta_for_color(
                                                    player1,
                                                    player2,
                                                    player3,
                                                    player4,
                                                    &colors,
                                                    &current,
                                                    change.money_delta,
                                                );
                                                history.with_mut(|h| h.push(snapshot));
                                            }
                                        },
                                        "{markets()[market_idx].shorts_cells[cell_idx].label}"
                                    }
                                    if markets()[market_idx].shorts_cells[cell_idx].is_painted() {
                                        div { class: "position-tooltip",
                                            span { class: "position-tooltip-label", "Sold" }
                                            span { class: "position-tooltip-price", "{markets()[market_idx].shorts_cells[cell_idx].remembered_price.unwrap_or_default()}" }
                                        }
                                    }
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
fn GameControls(
    show_modal: Signal<bool>,
    history: Signal<History>,
    player1: Signal<PlayerState>,
    player2: Signal<PlayerState>,
    player3: Signal<PlayerState>,
    player4: Signal<PlayerState>,
    player_colors: Signal<Vec<String>>,
    markets: Signal<Vec<MarketState>>,
    on_pay_percents: EventHandler<()>,
    on_pay_shorts_fee: EventHandler<()>,
    on_give_salary: EventHandler<()>,
    on_undo: EventHandler<()>,
    on_redo: EventHandler<()>,
) -> Element {
    let has_any_credit = player1().credit > 0 || player2().credit > 0 || player3().credit > 0 || player4().credit > 0;
    let has_any_shorts_fee = {
        let colors = player_colors();
        let current_markets = markets();
        colors.iter().any(|color| count_player_shorts(&current_markets, color) > 0)
    };

    rsx! {
        div { class: "game-controls",
            div { class: "game-control-actions",
                div { class: "top-actions",
                    div { class: "history-actions",
                        button {
                            class: "history-btn",
                            disabled: !history().can_undo(),
                            onclick: move |_| on_undo.call(()),
                            "◀"
                        }
                        button {
                            class: "history-btn",
                            disabled: !history().can_redo(),
                            onclick: move |_| on_redo.call(()),
                            "▶"
                        }
                    }
                    button {
                        class: "new-game-btn",
                        onclick: move |_| show_modal.set(true),
                        "Start new game"
                    }
                }
                div { class: "payment-actions",
                    button {
                        class: "pay-percents-btn",
                        disabled: !has_any_credit,
                        onclick: move |_| on_pay_percents.call(()),
                        "Pay %"
                    }
                    button {
                        class: "pay-shorts-fee-btn",
                        disabled: !has_any_shorts_fee,
                        onclick: move |_| on_pay_shorts_fee.call(()),
                        "Pay shorts fee"
                    }
                    button {
                        class: "give-salary-btn",
                        onclick: move |_| on_give_salary.call(()),
                        "Give salary"
                    }
                }
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
    mut fp_owner: Signal<usize>,
    mut fp_value: Signal<usize>,
    mut fp_drag_source: Signal<bool>,
    mut selected_color: Signal<String>,
    markets: Signal<Vec<MarketState>>,
    on_push_history: EventHandler<()>,
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
            let sold = count_player_shorts(std::slice::from_ref(market), &color);
            held * sell_price - sold * buy_price
        }).sum();
        base + stock_sum
    };

    rsx! {
        div { class: "player-panel",
            div {
                class: "player-title-row",
                ondragover: move |evt| {
                    if fp_drag_source() {
                        evt.prevent_default();
                    }
                },
                ondrop: move |evt| {
                    evt.prevent_default();
                    if fp_drag_source() && fp_owner() != player_idx {
                        on_push_history.call(());
                        fp_owner.set(player_idx);
                    }
                    fp_drag_source.set(false);
                },
                input {
                    class: "player-title-input",
                    value: "{state().name}",
                    oninput: move |evt| {
                        let value = evt.value();
                        state.with_mut(|s| s.name = value);
                    }
                }
                if fp_owner() == player_idx {
                    button {
                        class: "fp-btn",
                        draggable: true,
                        onclick: move |_| {
                            on_push_history.call(());
                            fp_value.with_mut(|value| *value = (*value % 3) + 1);
                        },
                        ondragstart: move |_| fp_drag_source.set(true),
                        ondragend: move |_| fp_drag_source.set(false),
                        "FP {fp_value()}"
                    }
                }
            }

            div { class: "player-row",
                span { "Credit:" }
                span { "{state().credit}" }
                button {
                    class: "control-btn credit-btn",
                    onclick: move |_| {
                        on_push_history.call(());
                        state.with_mut(PlayerState::subtract_credit);
                    },
                    "-"
                }
                button {
                    class: "control-btn credit-btn",
                    onclick: move |_| {
                        on_push_history.call(());
                        state.with_mut(PlayerState::add_credit);
                    },
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
                    onclick: move |_| {
                        if state().change_input.trim().parse::<i32>().is_ok() {
                            on_push_history.call(());
                        }
                        state.with_mut(PlayerState::apply_money);
                    },
                    "Apply"
                }
                button {
                    class: "control-btn apply-btn",
                    onclick: move |_| {
                        on_push_history.call(());
                        state.with_mut(PlayerState::apply_fee);
                    },
                    "Fee"
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

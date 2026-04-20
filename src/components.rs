use dioxus::prelude::*;
use crate::config::{PAINT_COLORS, PURPLE_COLOR};
use crate::market::{build_markets, shift_prices_cells_left, shift_prices_cells_right};
use crate::state::{MarketState, PlayerState};

#[component]
pub fn App() -> Element {
    let player1 = use_signal(|| PlayerState::with_name("Player 1"));
    let player2 = use_signal(|| PlayerState::with_name("Player 2"));
    let player3 = use_signal(|| PlayerState::with_name("Player 3"));
    let player4 = use_signal(|| PlayerState::with_name("Player 4"));
    let selected_color = use_signal(|| PAINT_COLORS[0].1.to_string());
    let player_colors = use_signal(|| {
        PAINT_COLORS.iter().map(|(_, c)| c.to_string()).collect::<Vec<_>>()
    });
    let drag_source: Signal<Option<usize>> = use_signal(|| None);
    let mut markets = use_signal(build_markets);

    rsx! {
        style { {include_str!("../assets/main.css")} }

        div { class: "app",
            div { class: "players",
                PlayerPanel { state: player1, player_idx: 0, player_colors, drag_source, selected_color, markets }
                PlayerPanel { state: player2, player_idx: 1, player_colors, drag_source, selected_color, markets }
                PlayerPanel { state: player3, player_idx: 2, player_colors, drag_source, selected_color, markets }
                PlayerPanel { state: player4, player_idx: 3, player_colors, drag_source, selected_color, markets }
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
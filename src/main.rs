use dioxus::prelude::*;

const BASE_COLOR: &str = "#78909C";
const HIGHLIGHT_COLOR: &str = "#90CAF9";
const PURPLE_COLOR: &str = "#AB47BC";

const PAINT_COLORS: [(&str, &str); 4] = [
    ("Red", "#EF5350"),
    ("Blue", "#42A5F5"),
    ("Green", "#66BB6A"),
    ("Yellow", "#FFEE58"),
];

struct MarketConfig {
    title: &'static str,
    bg_color: &'static str,
    right_count: usize,
    arrows: &'static [usize],
    default_purple_cells: &'static [usize],
}

const MARKET_CONFIGS: [MarketConfig; 6] = [
    MarketConfig {
        title: "Gold",
        bg_color: "#f2ad3d",
        right_count: 16,
        arrows: &[4, 8, 11, 14, 16],
        default_purple_cells: &[6, 7],
    },
    MarketConfig {
        title: "Oil",
        bg_color: "#141413",
        right_count: 16,
        arrows: &[4, 8, 12, 16],
        default_purple_cells: &[7, 8],
    },
    MarketConfig {
        title: "Nasdaq",
        bg_color: "#3d57ad",
        right_count: 15,
        arrows: &[3, 6, 9, 12, 15],
        default_purple_cells: &[8, 9],
    },
    MarketConfig {
        title: "Dow Jones",
        bg_color: "#6dcfcb",
        right_count: 14,
        arrows: &[4, 8, 11, 14],
        default_purple_cells: &[8, 9],
    },
    MarketConfig {
        title: "Bonds",
        bg_color: "#62b85e",
        right_count: 15,
        arrows: &[5, 10, 15],
        default_purple_cells: &[9, 10],
    },
    MarketConfig {
        title: "Country Stocks",
        bg_color: "#d92121",
        right_count: 12,
        arrows: &[2, 5, 8, 12],
        default_purple_cells: &[5, 6],
    },
];

#[derive(Clone)]
struct PlayerState {
    name: String,
    money: i32,
    credit: i32,
    change_input: String,
}

impl PlayerState {
    fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            money: 20,
            credit: 0,
            change_input: String::new(),
        }
    }

    fn apply_money(&mut self) {
        if let Ok(delta) = self.change_input.trim().parse::<i32>() {
            self.money += delta;
            self.change_input.clear();
        }
    }

    fn add_credit(&mut self) {
        self.money += 10;
        self.credit += 1;
    }

    fn subtract_credit(&mut self) {
        if self.money >= 10 && self.credit > 0 {
            self.money -= 10;
            self.credit -= 1;
        }
    }
}

#[derive(Clone)]
struct CellState {
    label: String,
    is_highlight: bool,
    color: String,
    is_arrow: bool,
}

impl CellState {
    fn paint(&mut self, selected_color: &str) {
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

#[derive(Clone)]
struct MarketState {
    title: &'static str,
    bg_color: &'static str,
    left_cells: Vec<CellState>,
    upper_cells: Vec<CellState>,
    lower_cells: Vec<CellState>,
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
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
                                    shift_left_cells_left(&mut m[market_idx].left_cells);
                                });
                            },
                            "◀"
                        }
                        div { class: "left-scale",
                            for cell_idx in 0..markets()[market_idx].left_cells.len() {
                                button {
                                    class: "price-cell",
                                    style: "background-color: {markets()[market_idx].left_cells[cell_idx].color};",
                                    onclick: move |_| {
                                        let current = selected_color();
                                        markets.with_mut(|m| {
                                            m[market_idx].left_cells[cell_idx].paint(&current);
                                        });
                                    },
                                    "{markets()[market_idx].left_cells[cell_idx].label}"
                                }
                            }
                        }
                        button {
                            class: "shift-btn",
                            onclick: move |_| {
                                markets.with_mut(|m| {
                                    shift_left_cells_right(&mut m[market_idx].left_cells);
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
                            for cell_idx in 0..markets()[market_idx].upper_cells.len() {
                                button {
                                    class: if markets()[market_idx].upper_cells[cell_idx].is_arrow { "cell arrow-gap" } else { "cell" },
                                    style: "background-color: {markets()[market_idx].upper_cells[cell_idx].color};",
                                    onclick: move |_| {
                                        let current = selected_color();
                                        markets.with_mut(|m| {
                                            m[market_idx].upper_cells[cell_idx].paint(&current);
                                        });
                                    },
                                    "{markets()[market_idx].upper_cells[cell_idx].label}"
                                }
                            }
                        }

                        div { class: "arrow-row",
                            for cell_idx in 0..markets()[market_idx].lower_cells.len() {
                                button {
                                    class: if markets()[market_idx].lower_cells[cell_idx].is_arrow { "cell arrow-gap" } else { "cell" },
                                    style: "background-color: {markets()[market_idx].lower_cells[cell_idx].color};",
                                    onclick: move |_| {
                                        let current = selected_color();
                                        markets.with_mut(|m| {
                                            m[market_idx].lower_cells[cell_idx].paint(&current);
                                        });
                                    },
                                    "{markets()[market_idx].lower_cells[cell_idx].label}"
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
            let mut purple_indices: Vec<i32> = market.left_cells.iter()
                .filter(|c| c.color == PURPLE_COLOR)
                .filter_map(|c| c.label.parse::<i32>().ok())
                .collect();
            purple_indices.sort();
            let sell_price = purple_indices.first().copied().unwrap_or(0);
            let buy_price = purple_indices.last().copied().unwrap_or(0);
            let held = market.upper_cells.iter().filter(|c| c.color == color).count() as i32;
            let sold = market.lower_cells.iter().filter(|c| c.color == color).count() as i32;
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

fn build_markets() -> Vec<MarketState> {
    MARKET_CONFIGS
        .iter()
        .map(|cfg| {
            let left_cells = (0..18)
                .map(|i| {
                    let is_highlight = matches!(i, 0 | 1 | 2 | 3 | 16 | 17);
                    let color = if cfg.default_purple_cells.contains(&i) {
                        PURPLE_COLOR
                    } else if is_highlight {
                        HIGHLIGHT_COLOR
                    } else {
                        BASE_COLOR
                    };

                    CellState {
                        label: i.to_string(),
                        is_highlight,
                        color: color.to_string(),
                        is_arrow: false,
                    }
                })
                .collect();

            let upper_cells = build_arrow_row(cfg.right_count, cfg.arrows, "↗");
            let lower_cells = build_arrow_row(cfg.right_count, cfg.arrows, "↘");

            MarketState {
                title: cfg.title,
                bg_color: cfg.bg_color,
                left_cells,
                upper_cells,
                lower_cells,
            }
        })
        .collect()
}

fn shift_left_cells_right(cells: &mut Vec<CellState>) {
    let n = cells.len();
    if cells[n - 1].color == PURPLE_COLOR {
        return;
    }
    for i in (0..n - 1).rev() {
        if cells[i].color == PURPLE_COLOR {
            let reset = if cells[i].is_highlight { HIGHLIGHT_COLOR } else { BASE_COLOR };
            cells[i + 1].color = PURPLE_COLOR.to_string();
            cells[i].color = reset.to_string();
        }
    }
}

fn shift_left_cells_left(cells: &mut Vec<CellState>) {
    let n = cells.len();
    if cells[0].color == PURPLE_COLOR {
        return;
    }
    for i in 1..n {
        if cells[i].color == PURPLE_COLOR {
            let reset = if cells[i].is_highlight { HIGHLIGHT_COLOR } else { BASE_COLOR };
            cells[i - 1].color = PURPLE_COLOR.to_string();
            cells[i].color = reset.to_string();
        }
    }
}

fn build_arrow_row(count: usize, arrows: &[usize], arrow_char: &str) -> Vec<CellState> {
    (1..=count)
        .map(|i| CellState {
            label: if arrows.contains(&i) {
                arrow_char.to_string()
            } else {
                String::new()
            },
            is_highlight: false,
            color: BASE_COLOR.to_string(),
            is_arrow: arrows.contains(&i),
        })
        .collect()
}

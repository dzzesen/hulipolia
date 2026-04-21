use crate::config::{BASE_COLOR, HIGHLIGHT_COLOR, PURPLE_COLOR, MARKET_CONFIGS};
use crate::state::{CellState, MarketState};

const AUTO_MIN_PRICE_INDEX: usize = 3;
const AUTO_MAX_PRICE_INDEX: usize = 16;

pub struct PositionChange {
    pub changed: bool,
    pub money_delta: i32,
}

pub fn build_markets() -> Vec<MarketState> {
    MARKET_CONFIGS
        .iter()
        .map(|cfg| {
            let prices_cells = (0..18)
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
                        remembered_price: None,
                    }
                })
                .collect();

            let holdings_cells = build_arrow_row(cfg.right_count, cfg.arrows, "↗");
            let shorts_cells = build_arrow_row(cfg.right_count, cfg.arrows, "↘");

            MarketState {
                title: cfg.title.to_string(),
                bg_color: cfg.bg_color.to_string(),
                prices_cells,
                holdings_cells,
                shorts_cells,
            }
        })
        .collect()
}

pub fn shift_prices_cells_right(prices_cells: &mut Vec<CellState>) {
    let n = prices_cells.len();
    if prices_cells[n - 1].color == PURPLE_COLOR {
        return;
    }
    for i in (0..n - 1).rev() {
        if prices_cells[i].color == PURPLE_COLOR {
            let reset = if prices_cells[i].is_highlight { HIGHLIGHT_COLOR } else { BASE_COLOR };
            prices_cells[i + 1].color = PURPLE_COLOR.to_string();
            prices_cells[i].color = reset.to_string();
        }
    }
}

pub fn shift_prices_cells_left(prices_cells: &mut Vec<CellState>) {
    let n = prices_cells.len();
    if prices_cells[0].color == PURPLE_COLOR {
        return;
    }
    for i in 1..n {
        if prices_cells[i].color == PURPLE_COLOR {
            let reset = if prices_cells[i].is_highlight { HIGHLIGHT_COLOR } else { BASE_COLOR };
            prices_cells[i - 1].color = PURPLE_COLOR.to_string();
            prices_cells[i].color = reset.to_string();
        }
    }
}

fn purple_price_indices(prices_cells: &[CellState]) -> Vec<usize> {
    prices_cells
        .iter()
        .enumerate()
        .filter(|(_, cell)| cell.color == PURPLE_COLOR)
        .map(|(idx, _)| idx)
        .collect()
}

fn current_buy_and_sell_prices(prices_cells: &[CellState]) -> (i32, i32) {
    let mut purple_indices: Vec<i32> = purple_price_indices(prices_cells)
        .into_iter()
        .map(|idx| idx as i32)
        .collect();
    purple_indices.sort();

    let sell_price = purple_indices.first().copied().unwrap_or(0);
    let buy_price = purple_indices.last().copied().unwrap_or(0);

    (buy_price, sell_price)
}

fn shift_prices_cells_right_auto(prices_cells: &mut Vec<CellState>) {
    let purple_indices = purple_price_indices(prices_cells);
    if purple_indices
        .last()
        .copied()
        .is_some_and(|idx| idx >= AUTO_MAX_PRICE_INDEX)
    {
        return;
    }

    shift_prices_cells_right(prices_cells);
}

fn shift_prices_cells_left_auto(prices_cells: &mut Vec<CellState>) {
    let purple_indices = purple_price_indices(prices_cells);
    if purple_indices
        .first()
        .copied()
        .is_some_and(|idx| idx <= AUTO_MIN_PRICE_INDEX)
    {
        return;
    }

    shift_prices_cells_left(prices_cells);
}

fn count_painted_arrow_cells(cells: &[CellState]) -> i32 {
    cells.iter()
        .filter(|cell| cell.is_arrow && cell.is_painted())
        .count() as i32
}

fn apply_price_shift_delta(prices_cells: &mut Vec<CellState>, delta: i32) {
    if delta > 0 {
        for _ in 0..delta {
            shift_prices_cells_right_auto(prices_cells);
        }
    } else {
        for _ in 0..(-delta) {
            shift_prices_cells_left_auto(prices_cells);
        }
    }
}

fn compact_position_cells(cells: &mut [CellState]) {
    let painted_positions: Vec<(String, Option<i32>)> = cells
        .iter()
        .filter(|cell| cell.is_painted())
        .map(|cell| (cell.color.clone(), cell.remembered_price))
        .collect();

    for (idx, cell) in cells.iter_mut().enumerate() {
        if let Some((color, remembered_price)) = painted_positions.get(idx) {
            cell.color = color.clone();
            cell.remembered_price = *remembered_price;
        } else {
            cell.reset_color();
        }
    }
}

fn remove_first_matching_position(cells: &mut [CellState], selected_color: &str) -> bool {
    if let Some(cell) = cells.iter_mut().find(|cell| cell.color == selected_color) {
        cell.reset_color();
        compact_position_cells(cells);
        return true;
    }

    false
}

fn toggle_and_compact_position_cell(
    cells: &mut [CellState],
    cell_idx: usize,
    selected_color: &str,
    remembered_price: i32,
) -> bool {
    if cells[cell_idx].is_painted() && cells[cell_idx].color != selected_color {
        return false;
    }

    cells[cell_idx].paint_position(selected_color, remembered_price);
    compact_position_cells(cells);
    true
}

fn sync_prices_after_holdings_change(market: &mut MarketState, painted_arrows_before: i32) {
    let painted_arrows_after = count_painted_arrow_cells(&market.holdings_cells);
    apply_price_shift_delta(
        &mut market.prices_cells,
        painted_arrows_after - painted_arrows_before,
    );
}

fn sync_prices_after_shorts_change(market: &mut MarketState, painted_arrows_before: i32) {
    let painted_arrows_after = count_painted_arrow_cells(&market.shorts_cells);
    apply_price_shift_delta(
        &mut market.prices_cells,
        painted_arrows_before - painted_arrows_after,
    );
}

pub fn paint_holdings_or_clear_shorts(
    market: &mut MarketState,
    cell_idx: usize,
    selected_color: &str,
) -> PositionChange {
    let holdings_arrow_count = count_painted_arrow_cells(&market.holdings_cells);
    let shorts_arrow_count = count_painted_arrow_cells(&market.shorts_cells);
    let (buy_price, sell_price) = current_buy_and_sell_prices(&market.prices_cells);

    if market.holdings_cells[cell_idx].color != selected_color
        && remove_first_matching_position(&mut market.shorts_cells, selected_color)
    {
        sync_prices_after_shorts_change(market, shorts_arrow_count);
        return PositionChange {
            changed: true,
            money_delta: -buy_price,
        };
    }

    let was_painted = market.holdings_cells[cell_idx].color == selected_color;
    let changed = toggle_and_compact_position_cell(
        &mut market.holdings_cells,
        cell_idx,
        selected_color,
        buy_price,
    );
    if changed {
        sync_prices_after_holdings_change(market, holdings_arrow_count);
    }
    PositionChange {
        changed,
        money_delta: if !changed {
            0
        } else if was_painted {
            sell_price
        } else {
            -buy_price
        },
    }
}

pub fn paint_shorts_or_clear_holdings(
    market: &mut MarketState,
    cell_idx: usize,
    selected_color: &str,
) -> PositionChange {
    let holdings_arrow_count = count_painted_arrow_cells(&market.holdings_cells);
    let shorts_arrow_count = count_painted_arrow_cells(&market.shorts_cells);
    let (buy_price, sell_price) = current_buy_and_sell_prices(&market.prices_cells);

    if market.shorts_cells[cell_idx].color != selected_color
        && remove_first_matching_position(&mut market.holdings_cells, selected_color)
    {
        sync_prices_after_holdings_change(market, holdings_arrow_count);
        return PositionChange {
            changed: true,
            money_delta: sell_price,
        };
    }

    let was_painted = market.shorts_cells[cell_idx].color == selected_color;
    let changed = toggle_and_compact_position_cell(
        &mut market.shorts_cells,
        cell_idx,
        selected_color,
        sell_price,
    );
    if changed {
        sync_prices_after_shorts_change(market, shorts_arrow_count);
    }
    PositionChange {
        changed,
        money_delta: if !changed {
            0
        } else if was_painted {
            -buy_price
        } else {
            sell_price
        },
    }
}

pub fn build_arrow_row(count: usize, arrows: &[usize], arrow_char: &str) -> Vec<CellState> {
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
            remembered_price: None,
        })
        .collect()
}

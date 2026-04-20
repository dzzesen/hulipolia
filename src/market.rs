use crate::config::{BASE_COLOR, HIGHLIGHT_COLOR, PURPLE_COLOR, MARKET_CONFIGS};
use crate::state::{CellState, MarketState};

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

fn compact_position_cells(cells: &mut [CellState]) {
    let painted_colors: Vec<String> = cells
        .iter()
        .filter(|cell| cell.is_painted())
        .map(|cell| cell.color.clone())
        .collect();

    for (idx, cell) in cells.iter_mut().enumerate() {
        if let Some(color) = painted_colors.get(idx) {
            cell.color = color.clone();
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

fn toggle_and_compact_position_cell(cells: &mut [CellState], cell_idx: usize, selected_color: &str) -> bool {
    if cells[cell_idx].is_painted() && cells[cell_idx].color != selected_color {
        return false;
    }

    cells[cell_idx].paint(selected_color);
    compact_position_cells(cells);
    true
}

pub fn paint_holdings_or_clear_shorts(
    market: &mut MarketState,
    cell_idx: usize,
    selected_color: &str,
) -> bool {
    if market.holdings_cells[cell_idx].color != selected_color
        && remove_first_matching_position(&mut market.shorts_cells, selected_color)
    {
        return true;
    }

    toggle_and_compact_position_cell(&mut market.holdings_cells, cell_idx, selected_color)
}

pub fn paint_shorts_or_clear_holdings(
    market: &mut MarketState,
    cell_idx: usize,
    selected_color: &str,
) -> bool {
    if market.shorts_cells[cell_idx].color != selected_color
        && remove_first_matching_position(&mut market.holdings_cells, selected_color)
    {
        return true;
    }

    toggle_and_compact_position_cell(&mut market.shorts_cells, cell_idx, selected_color)
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
        })
        .collect()
}

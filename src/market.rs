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
                title: cfg.title,
                bg_color: cfg.bg_color,
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
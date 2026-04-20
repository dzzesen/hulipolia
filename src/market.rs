use crate::config::{BASE_COLOR, HIGHLIGHT_COLOR, PURPLE_COLOR, MARKET_CONFIGS};
use crate::state::{CellState, MarketState};

pub fn build_markets() -> Vec<MarketState> {
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

pub fn shift_left_cells_right(cells: &mut Vec<CellState>) {
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

pub fn shift_left_cells_left(cells: &mut Vec<CellState>) {
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
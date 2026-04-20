pub const BASE_COLOR: &str = "#78909C";
pub const HIGHLIGHT_COLOR: &str = "#90CAF9";
pub const PURPLE_COLOR: &str = "#AB47BC";

pub const PAINT_COLORS: [(&str, &str); 4] = [
    ("Red", "#EF5350"),
    ("Blue", "#42A5F5"),
    ("Green", "#66BB6A"),
    ("Yellow", "#FFEE58"),
];

pub struct MarketConfig {
    pub title: &'static str,
    pub bg_color: &'static str,
    pub right_count: usize,
    pub arrows: &'static [usize],
    pub default_purple_cells: &'static [usize],
}

pub const MARKET_CONFIGS: [MarketConfig; 6] = [
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
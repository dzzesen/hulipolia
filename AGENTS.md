# AGENTS.md - Project Guidelines

## Project Overview

**Hulipoliya** is a web-based board game application built with Dioxus. It's a stock market simulation game where players trade in different markets (Gold, Oil, Nasdaq, Dow Jones, Bonds, Country Stocks) and track their money, credit, and capital.

## Technology Stack

- **Language**: Rust (Edition 2021)
- **Framework**: Dioxus (Web target)
- **Build Tool**: Cargo
- **Styling**: CSS (see `assets/main.css`)
- **Package Manager**: Cargo (Rust)

## Architecture

### Project Structure

```
hulipoliya/
├── Cargo.toml          # Rust dependencies
├── Dioxus.toml         # Dioxus configuration
├── shell.nix           # Nix shell configuration
├── assets/
│   └── main.css        # Application styles
├── src/
│   └── main.rs         # Main application code
└── target/             # Build output
```

### Key Components

1. **PlayerState** - Tracks player name, money, credit, and change input
2. **CellState** - Represents individual cells in the market grids
3. **MarketState** - Contains market configuration and cell states
4. **MarketConfig** - Static configuration for each market type

### Data Flow

- Signals are used for reactive state management
- Player panels update based on market state changes
- Cell painting affects player capital calculations
- Drag-and-drop for color swapping between players

## Coding Guidelines

### Rust Style

- Use `snake_case` for functions and variables
- Use `PascalCase` for types and components
- Use `SCREAMING_SNAKE_CASE` for constants
- Add `&'static` for string literals that don't change
- Prefer `use_signal` for reactive state in components

### Component Patterns

```rust
#[component]
fn ComponentName(prop: Type) -> Element {
    rsx! {
        // Component JSX
    }
}
```

### State Management

- Use `use_signal` for component-local state
- Pass signals to child components via props
- Use `.with_mut()` for mutable updates
- Clone signals when needed in closures

### Styling

- CSS classes are defined in `assets/main.css`
- Inline styles are used for dynamic colors: `style: "background-color: {value};"`
- Tailwind-style class names preferred

## Build Commands

```bash
# Development server
dx serve

# Build for release
dx build --release

# Build with Cargo
cargo build
```

## Notes for Agents

- This is a single-page application with no routing
- State is ephemeral (no persistence layer)
- UI is grid-based with fixed layouts
- Color values are hardcoded as hex strings
- Player count is fixed at 4

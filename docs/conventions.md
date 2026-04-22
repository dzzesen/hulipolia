# Hulipolia Naming Conventions

This document defines the main concepts and naming conventions used in the Hulipolia codebase to ensure clarity and consistency when discussing the game's mechanics.

## Core Concepts

### Market Components

#### `prices_cells`
The vertical column of numbered cells on the **left side** of each market.  
This represents the price levels for that market. The purple markers on this scale indicate the current buy and sell prices.

- **Location**: Left side of the game field
- **Appearance**: Column of numbers (0-17)
- **Purpose**: Shows available price levels
- **Key behavior**: Can be shifted left/right to move price markers

#### `holdings_cells`
The row of cells on the **top line** of the right section of each market.  
These cells represent stocks that players have **bought** (long positions). When a player paints a cell here with their color, it means they own that position at the corresponding price.

- **Location**: Top row of the right section (above the arrows)
- **Appearance**: Row of empty cells with occasional arrow indicators (↗)
- **Purpose**: Track which stocks the player has purchased
- **Key behavior**: Painted with player colors to mark ownership

#### `shorts_cells`
The row of cells on the **bottom line** of the right section of each market.  
These cells represent stocks that players have **sold** (short positions). When a player paints a cell here with their color, it means they have sold/short-sold that position.

- **Location**: Bottom row of the right section (below the arrows)
- **Appearance**: Row of empty cells with occasional arrow indicators (↘)
- **Purpose**: Track which stocks the player has sold
- **Key behavior**: Painted with player colors to mark short positions

## Player State

### `money`
The player's liquid cash on hand.

### `credit`
The player's credit/loans taken. Each credit unit represents 10 units of borrowed money.

### `capital`
Calculated as: `money - (10 * credit) + sum of all holdings_cells - sum of all shorts_cells`  
This represents the player's total net worth including all stock positions.

## Code Naming Conventions

| Concept | Variable Name | Description |
|---------|---------------|-------------|
| Price scale | `prices_cells` | Price levels on the left |
| Bought positions | `holdings_cells` | Bought/owned positions (top row) |
| Sold positions | `shorts_cells` | Sold/short positions (bottom row) |

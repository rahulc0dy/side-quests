# Cit Cat Coe

Cit Cat Coe is a browser-based game inspired by Tic Tac Toe with a twist. Each player—represented by circles and crosses—has a limited number of moves available on the board. Once three moves are played by a player, the oldest move is removed when a new one is made. The goal is to line up three markers in a row.

## How to Play

- **Players:** Two players take turns. One uses the circle and the other uses the cross.
- **Gameplay:**
  - Click on an empty cell to place your marker.
  - Each player can have a maximum of three markers on the board. If a player already has three, placing a new marker will automatically remove their oldest marker.
  - The game checks for a winning combination after each move.
- **Winning:** As soon as one player aligns three of their markers (horizontally, vertically, or diagonally), an alert will announce the winner and the game will reset.

## Running the Game

1. Open the `index.html` file in your browser.
2. Click on the grid cells to start playing.
3. Enjoy the unique twist on a classic game!

## Code Structure

- **HTML/CSS:** The layout is defined using a simple grid and custom styles, with CSS variables enabling easy customization.
- **JavaScript:** Handles player turns, move management (including the removal of oldest moves), and win detection based on preset winning combinations.

Have fun playing Cit Cat Coe!

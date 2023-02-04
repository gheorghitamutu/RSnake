use crossterm::{
    cursor, execute, queue, style, terminal, QueueableCommand,
};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize terminal and set it to raw mode
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, crossterm::cursor::Hide).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    // Initialize the game state
    let mut x = 10;
    let mut y = 10;
    let mut width = 50;
    let mut height = 20;
    let mut snake = vec![(x, y)];

    // Main game loop
    loop {
        // Clear the screen
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

        // Draw the snake
        for (x, y) in snake.iter() {
            queue!(
                stdout,
                cursor::MoveTo(*x, *y),
                style::SetBackgroundColor(style::Color::Green),
                style::Print(" "),
                style::SetBackgroundColor(style::Color::Reset),
            )
            .unwrap();
        }

        // Update the game state
        x += 1;
        y += 1;
        snake.push((x, y));

        // Remove the tail of the snake if it has grown too long
        while snake.len() > 10 {
            snake.remove(0);
        }

        // Check for screen boundaries and wrap around if necessary
        if x >= width {
            x = 0;
        }
        if y >= height {
            y = 0;
        }

        // Flush the stdout buffer to display the changes
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(100));
    }
}

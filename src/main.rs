use std::io::{ self, stdout, Write as _ };
use std::time::Duration;
use tokio_stream::StreamExt; 
use crossterm::{
    cursor,
    terminal,
    event::{ Event, EventStream, KeyEvent, KeyCode },
    execute,
    ExecutableCommand as _,
};

mod assets;
use assets::*;

const FPS: f64 = 8.0;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Transmitter, receiver.
    let (tx, rx) = tokio::sync::watch::channel(false);

    let input_handler = tokio::spawn(handle_key(tx));
    let renderer = tokio::spawn(render(rx));

    // Safety: these shouldn't panic.
    let (r1, r2) = tokio::join!(input_handler, renderer);
    r1?; r2??;
    Ok(())
}

async fn handle_key(tx: tokio::sync::watch::Sender<bool>) {                          
    let mut stream = EventStream::new();
    while !matches!(
        stream.next().await,
        Some(Ok(Event::Key(KeyEvent {
            code: KeyCode::Enter
                | KeyCode::Esc
                | KeyCode::Char(_),
            ..
        })))
    ) {}

    tx.send(true)
      .expect("No one was listening");
}

async fn render(mut rx: tokio::sync::watch::Receiver<bool>) -> io::Result<()> {
    let mut stdout = stdout();
    let delta: Duration = Duration::from_secs_f64(1.0 / FPS);

    terminal::enable_raw_mode()?;
    execute!(stdout,
             cursor::Hide,
             terminal::EnterAlternateScreen)?;

    let mut ticker = tokio::time::interval(delta);

    for &(x, y) in FRAME_ORDER.iter().cycle() {
        let frame = FRAMES[x][y];
        // TODO: see terminal::BeginSynchronizedUpdate
        stdout.execute(cursor::MoveTo(0, 0))?;

        for line in frame.lines() {
            print!("{line}\r\n");
        }
        stdout.flush()?;

        tokio::select! {
            // Safety: we don't care if there's a transmitter or not.
            _ = rx.changed() => break,
            _ = ticker.tick() => {}
        }

        // Clean the screen
        stdout.execute(cursor::MoveTo(0, 0))?;
        for line in frame.lines() {
            print!("{}\r\n", " ".repeat(line.len()));
        }
    }

    execute!(stdout,
             terminal::LeaveAlternateScreen,
             cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}


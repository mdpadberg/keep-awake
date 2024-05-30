use enigo::{Direction, Enigo, Key, Keyboard};
use std::{thread, time::Duration};

pub fn keyboard(enigo: &mut Enigo) -> anyhow::Result<()> {
    enigo.key(Key::Meta, Direction::Press)?;
    enigo.key(Key::Tab, Direction::Press)?;
    thread::sleep(Duration::from_millis(200));
    enigo.key(Key::Meta, Direction::Release)?;
    enigo.key(Key::Tab, Direction::Release)?;
    Ok(())
}

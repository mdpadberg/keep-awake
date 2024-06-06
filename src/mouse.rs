use std::{thread, time::Duration};

use enigo::{Coordinate, Enigo, Mouse};
use rand::{thread_rng, Rng};

pub fn mouse(enigo: &mut Enigo) -> anyhow::Result<()> {
    let mut i = 0;
    let mut rng = thread_rng();
    while i < 10 {
        let move_x: i32 = rng.gen_range(-150..150);
        let move_y: i32 = rng.gen_range(-150..150);
        enigo.move_mouse(move_x, move_y, Coordinate::Rel)?;
        i += 1;
        thread::sleep(Duration::from_millis(200));
    }
    Ok(())
}

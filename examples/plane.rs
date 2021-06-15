//! Shows how you can manipulate [`Plane`]s.

use notcurses::sys::NcChannelPairMethods; // TEMP
use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Nc::new()?;

    let mut p1 = Plane::build().rows(20).cols(40).new_pile(&mut nc)?;
    p1.set_base("X", Style::BOLD | Style::ITALIC, 0)?;
    p1.render_raster()?;
    sleep![0, 500];

    let mut p2 = Plane::build()
        .rows(10)
        .cols(10)
        .y(2)
        .x(3)
        .in_pile(&mut p1)?;
    p2.set_base(
        "·",
        Style::REVERSE,
        sys::NcChannelPair::with_rgb(0xaadd2b, 0x882222),
    )?;
    p1.render_raster()?;
    sleep![0, 500];

    for _ in 0..10 {
        p2.move_rel(1, 3)?;
        p1.render_raster()?;
        sleep![0, 50];
    }
    sleep![2];

    Ok(())
}

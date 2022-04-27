// notcurses::examples::plane
//
//!
//

#![allow(unused_variables, unused_mut)]

use notcurses::*;

fn main() -> Result<()> {
    let mut nc = Notcurses::new_cli_silent()?;

    // # size

    // # position

    // let's create a root plane at 1,1 with a child at 2,2:
    let mut p1 = Plane::new_at(&mut nc, 1, 1)?;
    let p2 = p1.new_child_at(2, 2)?;

    // check their position relative to their parent:
    assert_eq![p1.yx(), Offset(1, 1)];
    assert_eq![p2.yx(), Offset(2, 2)];

    // check the absolute position, which is relative to the root of their pile:
    assert_eq![p1.abs_yx(), Offset(1, 1)];
    assert_eq![p2.abs_yx(), Offset(3, 3)];

    // # translate WIP

    // let's create a square of 5×5 at 10,10:
    let p1 = Plane::new_sized_at(&mut nc, Size(5, 5), 10, 10)?;

    // check top-left and bottom-right square coordinates are inside the plane:
    assert_eq![p1.translate_abs((10, 10)), (Offset(0, 0), true)];
    assert_eq![p1.translate_abs((14, 14)), (Offset(4, 4), true)];

    // some other coordinates outside the plane:
    assert_eq![p1.translate_abs((2, 2)), (Offset(-8, -8), false)];
    assert_eq![p1.translate_abs((20, 20)), (Offset(10, 10), false)];

    if p1.translate_abs((20, 20)).1 {
        println!("yes");
    } else {
        println!("no");
    }

    // let abs = p1.translate_abs(12, 9);
    // println!("{abs:?}");

    // # cursor

    Ok(())
}

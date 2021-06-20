use crate::{sys::NcCell, Plane, Style};

/// Part of a [`Cell`].
pub const BACKSTOP: u8 = 0;

/// A `u128` of [`char`] + [`BACKSTOP`] + *width* + [`Style`] + [`Channels`],
/// part of a [`Plane`][crate::Plane].
///
/// # Diagram
/// ```txt
/// CCCCCCCC CCCCCCCC CCCCCCCC CCCCCCCC  char
/// BBBBBBBB WWWWWWWW 11111111 11111111  BACKSTOP + width + Style
/// ~~AA~~~~ RRRRRRRR GGGGGGGG BBBBBBBB  Foreground Channel
/// ~~AA~~~~ RRRRRRRR GGGGGGGG BBBBBBBB  Background Channel
/// ```
pub struct Cell {
    raw: NcCell,
}

impl Cell {
    /// Returns the `char`.
    pub fn egc<'a>(&mut self, plane: &mut Plane<'a>) -> char {
        self.raw.egc(plane.as_ncplane_mut())
    }

    /// Returns the [`Styles`].
    pub fn styles<'a>(&mut self) -> Style {
        self.raw.styles().into()
    }

    /// Adds the specified [`Style`]s.
    pub fn add_styles(&mut self, styles: Style) {
        self.raw.styles_on(styles.bits())
    }

    /// Removes the specified [`Style`]s.
    pub fn remove_styles(&mut self, styles: Style) {
        self.raw.styles_off(styles.bits())
    }

    /// Sets just the specified [`Style`]s.
    pub fn set_styles(&mut self, styles: Style) {
        self.raw.styles_set(styles.bits())
    }

    // pub fn channels(&mut self) -> Channels {
    //     self.0.channels()
    // }
}
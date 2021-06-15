//!
//! - <https://notcurses.com/notcurses_visual.3.html>

#![allow(dead_code)]

// TODO
// - NcRgba, NcVGeom...
// - allow changing the inner options after, with a safe interface
// - add alpha_color NCVISUAL_OPTION_ADDALPHA
// - add halign & valign
// - add blend NCVISUAL_OPTION_BLEND
// - add nodegrade NCVISUAL_OPTION_NODEGRADE
//
// MAYBE
// - offer the alternative of using a VisualOptions structure. (old: visual3)

use crate::sys::{self, NcVisual, NcVisualOptions};
use crate::{ncresult, Dimension, Nc, Result};

mod blitter;
mod builder;
mod scale;

pub use blitter::Blitter;
pub use builder::VisualBuilder;
pub use scale::Scale;

/// A virtual pixel framebuffer.
///
/// Internally, a `Visual` wraps an [`NcVisual`] and [`NcVisualOptions`], because
/// the options are only used by the [`geom`][Visual#method.geom],
/// [`render`][Visual#method.render] and
/// [`simple_streamer`][Visual#simple_streamer] methods, and not for the
/// creation of the `Visual` itself.
#[derive(Debug)]
pub struct Visual<'a> {
    pub(crate) raw: &'a mut NcVisual,
    pub(crate) options: NcVisualOptions,
}

impl<'a> Drop for Visual<'a> {
    /// Destroys the Visual.
    ///
    /// Rendered elements will not be disrupted, but the visual can be neither
    /// decoded nor rendered any further.
    fn drop(&mut self) {
        let _ = self.raw.destroy();
    }
}

/// # Methods
impl<'a, 'b> Visual<'a> {
    /// Returns a default [`VisualBuilder`] used to customize a new `Visual`.
    pub fn build() -> VisualBuilder<'a, 'b> {
        VisualBuilder::default()
    }

    // /// Creates a `Visual` from an existing [`NcVisual`] and [`NcVisualOptions`].
    // pub fn from_ncvisual(visual: &'a mut NcVisual) -> Visual<'a> {
    //     Self {
    //         raw: visual,
    //         // options: None,
    //     }
    // }

    /// Returns a mutable reference to the inner `NcVisual`.
    pub fn as_ncvisual(&'a mut self) -> &'a mut NcVisual {
        self.raw
    }

    /// Resizes the visual to `x`,`y` pixels, using interpolation.
    pub fn resize(&mut self, x: Dimension, y: Dimension) -> Result<()> {
        ncresult![NcVisual::resize(self.raw, y, x)]
    }

    /// Resizes the visual to `x`,`y` pixels, without using interpolation.
    pub fn resize_ni(&mut self, x: Dimension, y: Dimension) -> Result<()> {
        ncresult![NcVisual::resize_noninterpolative(self.raw, y, x)]
    }

    /// Renders the decoded frame to the configured [`Plane`][crate::Plane].
    //
    // Here render doesn't return the plane.
    pub fn render(&mut self, nc: &mut Nc) -> Result<()> {
        let _ = NcVisual::render(self.raw, nc.raw, &self.options)?;
        Ok(())
    }
}

/// # Post-Builder Configuration Methods
///
/// These methods allows to re-configure a `Visual` after it has been built
/// via [`VisualBuilder`].
impl<'a, 'b> Visual<'a> {

    /// Sets the `Visual` based off RGBA content in memory at `rgba`.
    pub fn set_from_rgba(&mut self, rgba: &[u8], cols: Dimension, rows: Dimension) -> Result<()> {
        self.raw = NcVisual::from_rgba(rgba, rows, cols * 4, cols)?;
        Ok(())
    }

    /// Sets the `Visual` based off BGRA content in memory at `bgra`.
    pub fn set_from_bgra(&mut self, bgra: &[u8], cols: Dimension, rows: Dimension) -> Result<()> {
        self.raw = NcVisual::from_bgra(bgra, rows, cols * 4, cols)?;
        Ok(())
    }

    /// Sets the `Visual` from a `file`, extracts the codec and paramenters
    /// and decodes the first image to memory.
    pub fn set_from_file(&mut self, file: &str) -> Result<()> {
        self.raw = NcVisual::from_file(file)?;
        Ok(())
    }
    /// Sets the blitter
    pub fn set_blitter(&mut self, blitter: Blitter) {
        self.options.blitter = blitter.bits();
    }

    /// Sets whether the scaling should be done with interpolation or not.
    ///
    /// The default is to interpolate.
    pub fn set_interpolate(&mut self, interpolate: bool) {
        if interpolate {
            self.options.flags &= !sys::NCVISUAL_OPTION_NOINTERPOLATE as u64;
        } else {
            self.options.flags |= sys::NCVISUAL_OPTION_NOINTERPOLATE as u64;
        }
    }
}
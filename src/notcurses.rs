//! `Nc` wrapper struct and traits implementations.

use crate::{ncresult, sys::Notcurses, Capabilities, Dimension, Result};

/// The main **notcurses** context.
///
/// *A  wrapper around `sys::`[`Notcurses`].*
#[derive(Debug)]
pub struct Nc<'a> {
    pub(crate) raw: &'a mut Notcurses,
}

impl<'a> Drop for Nc<'a> {
    /// Destroys the Nc context.
    fn drop(&mut self) {
        let _ = self.raw.stop();
    }
}

impl<'a> Nc<'a> {
    /// New Notcurses instance.
    pub fn new() -> Result<Self> {
        Ok(Self {
            raw: Notcurses::new()?,
        })
    }

    /// New Notcurses instance, without an alternate screen.
    pub fn without_altscreen() -> Result<Self> {
        Ok(Self {
            raw: Notcurses::without_altscreen()?,
        })
    }

    // pub fn align
    // pub fn at_yx

    /// Disables the terminal cursor, if supported.
    pub fn cursor_disable(&mut self) -> Result<()> {
        ncresult![self.raw.cursor_disable()]
    }

    /// Enables the terminal cursor, if supported, plaxing it at `x`,`y`.
    pub fn cursor_enable(&mut self, x: Dimension, y: Dimension) -> Result<()> {
        ncresult![self.raw.cursor_enable(y, x)]
    }

    // debug
    // debug_caps

    /// Destroys all [`Plane`][crate::Plane]s.
    ///
    /// Any pre-existing `Planes` will be invalid and shouldn't be used again.
    pub fn drop_planes(&mut self) {
        self.raw.drop_planes();
    }

    // TODO:
    // getc
    // getc_nblock
    // getc_blocking
    // inputready_fd

    // lex_blitter
    // lex_margins
    // lex_scalemode

    /// Disables signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z). They are enabled by default.
    pub fn linesigs_disable(&mut self) -> Result<()> {
        ncresult![self.raw.linesigs_disable()]
    }

    /// Restores signals originating from the terminal's line discipline, i.e.
    /// SIGINT (^C), SIGQUIT (^), and SIGTSTP (^Z), if disabled.
    pub fn linesigs_enable(&mut self) -> Result<()> {
        ncresult![self.raw.linesigs_enable()]
    }

    /// Disables mouse events.
    ///
    /// Any events in the input queue can still be delivered.
    pub fn mouse_disable(&mut self) -> Result<()> {
        ncresult![self.raw.mouse_disable()]
    }

    /// Enable the mouse in "button-event tracking" mode with focus detection
    /// and UTF8-style extended coordinates.
    ///
    /// On success, mouse events will be published to [getc()][Nc#method.getc].
    pub fn mouse_enable(&mut self) -> Result<()> {
        ncresult![self.raw.mouse_enable()]
    }

    /// Refreshes the physical screen to match what was last rendered (i.e.,
    /// without reflecting any changes since the last call to
    /// [render][crate::Notcurses#method.render]).
    //
    // WIP
    //
    // This is primarily useful if the screen is externally corrupted, or if an
    // [NCKEY_RESIZE][crate::NCKEY_RESIZE] event has been read and you're not
    // yet ready to render.
    pub fn refresh(&mut self) -> Result<(Dimension, Dimension)> {
        ncresult![self.raw.refresh()]
    }

    // stats
    // stats_aloc
    // stats_reset

    // str_blitter
    // str_scalemode

    // TODO:
    // supported_styles

    /// Returns the capabilities of the terminal.
    pub fn term_capabilities(&self) -> Capabilities {
        Capabilities {
            halfblock: self.raw.canhalfblock(),
            quadrant: self.raw.canquadrant(),
            sextant: self.raw.cansextant(),
            braille: self.raw.canbraille(),
            utf8: self.raw.canutf8(),
            images: self.raw.canopen_images(),
            videos: self.raw.canopen_videos(),
            pixel: self.raw.check_pixel_support().unwrap_or(false),
            truecolor: self.raw.cantruecolor(),
            fade: self.raw.canfade(),
            palette_change: self.raw.canchangecolor(),
            palette_size: self.raw.palette_size().unwrap_or(0),
        }
    }

    /// Returns the size of the terminal in columns and rows (x, y).
    pub fn term_size(&self) -> (Dimension, Dimension) {
        self.raw.term_dim_yx()
    }

    /// Returns the name of the detected terminal.
    pub fn term_name(&self) -> String {
        self.raw.detected_terminal()
    }

    /// Returns a human-readable string describing the running notcurses version.
    pub fn version() -> String {
        Notcurses::version()
    }

    /// Returns the running Notcurses version components
    /// (major, minor, patch, tweak).
    pub fn version_components() -> (u32, u32, u32, u32) {
        Notcurses::version_components()
    }
}

// MAYBE:builder pattern -------------------------------------------------------

// pub struct NcBuilder {
// }
//
// impl<'a> NcBuilder {
//     pub fn with_altscreen() {
//     }
// }

/*
/// # Constructors and methods overriden from Notcurses
impl<'a> Nc<'a> {
    // wrap constructors

    // /// New Nc (without banners).
    // pub fn new() -> Result<Self> {
    //     raw_wrap![Notcurses::new()]
    // }
    //
    // /// New Nc, with banners.
    // pub fn with_banners() -> Result<Self> {
    //     raw_wrap![Notcurses::with_banners()]
    // }
    //
    // /// New Nc, without an alternate screen (nor banners).
    // pub fn without_altscreen() -> Result<Self> {
    //     raw_wrap![Notcurses::without_altscreen()]
    // }
    //
    // /// New Nc, expects `NCOPTION_*` flags.
    // pub fn with_flags(flags: u64) -> Result<Self> {
    //     raw_wrap![Notcurses::with_flags(flags)]
    // }
    //
    // /// New Nc, expects [NotcursesOptions].
    // pub fn with_options(options: NotcursesOptions) -> Result<Self> {
    //     raw_wrap![Notcurses::with_options(options)]
    // }
    //
    // /// New Nc, expects [NcLogLevel] and flags.
    // pub fn with_debug(loglevel: NcLogLevel, flags: u64) -> Result<Self> {
    //     raw_wrap![Notcurses::with_debug(loglevel, flags)]
    // }

    // disable destructor

    /// Since Nc already implements [Drop](#impl-Drop),
    /// this function is made no-op.
    pub fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    // wrap associated functions

    /// Returns the offset into `availcols` at which `cols` ought be output given
    /// the requirements of `align`.
    pub fn align(availcols: Dimension, align: NcAlign, cols: Dimension) -> Result<()> {
        Notcurses::align(availcols, align, cols)
    }

    /// Gets the name of an [NcBlitter] blitter.
    pub fn str_blitter(blitter: NcBlitter) -> String {
        Notcurses::str_blitter(blitter)
    }

    /// Gets the name of an [NcScale] scaling mode.
    pub fn str_scalemode(scalemode: NcScale) -> String {
        Notcurses::str_scalemode(scalemode)
    }

    /// Returns an [NcBlitter] from a string representation.
    pub fn lex_blitter(op: &str) -> Result<NcBlitter> {
        Notcurses::lex_blitter(op)
    }

    /// Lexes a margin argument according to the standard Notcurses definition.
    ///
    /// There can be either a single number, which will define all margins equally,
    /// or there can be four numbers separated by commas.
    ///
    pub fn lex_margins(op: &str, options: &mut NotcursesOptions) -> Result<()> {
        Notcurses::lex_margins(op, options)
    }

    /// Returns an [NcScale] from a string representation.
    pub fn lex_scalemode(op: &str) -> Result<NcScale> {
        Notcurses::lex_scalemode(op)
    }

    /// Returns a human-readable string describing the running Nc version.
    pub fn version() -> String {
        Notcurses::version()
    }

    /// Returns the running Notcurses version components
    /// (major, minor, patch, tweak).
    pub fn version_components() -> (u32, u32, u32, u32) {
        Notcurses::version_components()
    }
}
*/

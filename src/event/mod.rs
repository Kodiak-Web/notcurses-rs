// notcurses::event
//
//!
//

use crate::{InputType, Key, KeyMod, Position, Received};

/// An input event.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Event {
    /// A received [`Key`][crate::Key] or [`char`].
    pub received: Received,

    /// Keyboard modifiers.
    pub keymod: KeyMod,

    /// The type of the input
    pub itype: InputType,

    /// The cell position of the event, if defined.
    pub cell: Option<Position>,

    /// Pixel offset within the cell, if defined.
    pub offset: Option<Position>,
}

mod std_impls {
    use super::{Event, Position, Received};
    use crate::sys::NcInput;
    use std::fmt;

    impl fmt::Display for Event {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cell = if let Some(c) = self.cell { c.to_string() } else { "None".into() };
            let offset = if let Some(o) = self.offset { o.to_string() } else { "None".into() };
            write!(f,
                "{} {} {} {} {}",
                self.received, self.keymod, self.itype, cell, offset,
            )
        }
    }

    impl fmt::Debug for Event {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cell = if let Some(c) = self.cell { c.to_string() } else { "None".into() };
            let offset = if let Some(o) = self.offset { o.to_string() } else { "None".into() };
            write!(f,
                "Event {{received:{} mod:{} type:{} cell:{} offset:{} }}",
                self.received, self.keymod, self.itype, cell, offset,
            )
        }
    }

    impl From<(Received, NcInput)> for Event {
        fn from(received_input: (Received, NcInput)) -> Event {
            let (received, input) = received_input;

            // cell position & offset is only relevant for mouse events
            let (mut cell, mut offset) = (None, None);
            if let Received::Key(k) = received {
                if k.is_mouse() {
                    if input.y != -1 {
                        // != undefined
                        cell = Some(Position(input.y, input.x));
                    }
                    if input.ypx != -1 {
                        offset = Some(Position(input.ypx, input.xpx));
                    }
                }
            };

            Event {
                received,
                keymod: input.modifiers.into(),
                itype: input.evtype.into(),
                cell,
                offset,
            }
        }
    }
}

/// # methods
impl Event {
    /// Returns true if something has been received.
    pub fn is_received(&self) -> bool {
        self.received != Received::NoInput
    }

    /// Returns true if a [`Key`] has been received.
    pub fn has_key(&self) -> bool {
        matches!(self.received, Received::Key(_))
    }

    /// Returns true if a specific [`Key`] has been received.
    pub fn is_key(&self, key: Key) -> bool {
        matches!(self.received, Received::Key(k) if k == key)
    }

    /// Returns true if a [`char`] has been received.
    pub fn has_char(&self) -> bool {
        matches!(self.received, Received::Char(_))
    }

    /// Returns true if a specific [`char`] has been received.
    pub fn is_char(&self, character: char) -> bool {
        matches!(self.received, Received::Char(c) if c == character)
    }
}

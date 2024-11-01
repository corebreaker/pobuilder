//! Handling of [Uniforum Portable Objects][PO]
//!
//! This format is used by the well known [gettext] suite and also supported by the
//! [translate-toolkit][tt] suite. It is a simple text format storing translation units with
//! optional context and plural variants.
//!
//! For modern translation work it's disadvantage is the plural system only supports integers.
//!
//! [PO]: https://www.gnu.org/software/gettext/manual/html_node/PO-Files.html
//! [gettext]: https://www.gnu.org/software/gettext/
//! [tt]: http://toolkit.translatehouse.org/

mod decoder;
mod file;
mod line;
mod line_iter;
mod message_extractor;
mod parser;
mod unescape;

pub use self::{parser::PoParser, file::PoFile};

pub(super) use self::{decoder::Decoder, message_extractor::MessageExtractor};

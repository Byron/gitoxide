//! An implementation of negotiation algorithms to help the server figure out what we have in common so it can optimize
//! the pack it sends to only contain what we don't have.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

/// The way the negotiation is performed.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Algorithm {
    /// Do not send any information at all, likely at cost of larger-than-necessary packs.
    Noop,
    /// Walk over consecutive commits and check each one. This can be costly be assures packs are exactly the size they need to be.
    #[default]
    Consecutive,
    /// Like `Consecutive`, but skips commits to converge faster, at the cost of receiving packs that are larger than they have to be.
    Skipping,
}

// static int next_flush(int stateless_rpc, int count)
// {
// if (stateless_rpc) {
// if (count < LARGE_FLUSH)
// count <<= 1;
// else
// count = count * 11 / 10;
// } else {
// if (count < PIPESAFE_FLUSH)
// count <<= 1;
// else
// count += PIPESAFE_FLUSH;
// }
// return count;
// }
/// Calculate how many `HAVE` lines we may send in one round, with variation depending on whether the `transport_is_stateless` or not.
/// `window_size` is the previous (or initial) value of the window size.
pub fn window_size(transport_is_stateless: bool, window_size: impl Into<Option<usize>>) -> usize {
    let current_size = match window_size.into() {
        None => return 16,
        Some(cs) => cs,
    };
    const PIPESAFE_FLUSH: usize = 32;
    const LARGE_FLUSH: usize = 16384;

    if transport_is_stateless {
        if current_size < LARGE_FLUSH {
            current_size * 2
        } else {
            current_size * 11 / 10
        }
    } else {
        if current_size < PIPESAFE_FLUSH {
            current_size * 2
        } else {
            current_size + PIPESAFE_FLUSH
        }
    }
}

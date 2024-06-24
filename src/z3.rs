//! Re-export of z3 types and helper functions
use z3::{ast::Bool, Context, Model};

use crate::{util::unbits, Endianness};
pub use z3;

/// Evaluate all bits from the current context and return them as bytes
///
/// Assumes the bools are symbol 0..N in the given context
pub fn bits_to_bytes(ctx: &Context, model: &Model, endian: Endianness) -> Vec<u8> {
    let mut res = Vec::new();
    for i in 0.. {
        let Some(b) = model.eval(&Bool::new_const(&ctx, i), false) else {
            break;
        };
        let Some(b) = b.as_bool() else {
            break;
        };
        res.push(b);
    }
    unbits(&res, endian)
}

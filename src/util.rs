//! Small utility functions
use crate::Endianness;

/// Converts a list of bools to a byte vector
pub fn unbits(bools: &[bool], endian: Endianness) -> Vec<u8> {
    let mut res = Vec::new();

    for bs in bools.chunks_exact(8) {
        let mut cur = 0;
        for (i, &b) in bs.iter().enumerate() {
            let idx = match endian {
                Endianness::Little => i,
                Endianness::Big => 7 - i,
            };
            cur |= (b as u8) << idx;
        }

        res.push(cur);
    }

    res
}

use std::collections::BTreeSet;

use orthrus_core::prelude::*;

use super::{Permissions, Segment};
use crate::error::FerroxError;

pub struct DolBinary;

impl DolBinary {
    // This function assumes it's accepting valid data
    pub fn segments(data: &[u8]) -> Result<Vec<Segment<u32>>, FerroxError> {
        let mut data = DataCursorRef::new(data, Endian::Big);
        // Maximum we can have is 18 segments (7 text, 11 data), with bss overlapping the entire range
        let mut segments = Vec::with_capacity(37);

        let mut offsets = [0u32; 18];
        for n in 0..offsets.len() {
            offsets[n] = data.read_u32()?;
        }

        let mut addresses = [0u32; 18];
        for n in 0..addresses.len() {
            addresses[n] = data.read_u32()?;
        }

        let mut sizes = [0u32; 18];
        for n in 0..sizes.len() {
            sizes[n] = data.read_u32()?;
        }

        // Now we need to actually create segments
        for n in 0..7 {
            if sizes[n] > 0 {
                // Code segments
                segments.push(Segment {
                    address: addresses[n],
                    size: sizes[n],
                    offset: offsets[n],
                    // TODO: this is technically RWX since the SDK modifies some assembly when installing
                    // exceptions, do we bother enforcing that? "Real" programs will treat it as RX
                    permissions: Permissions::READ | Permissions::EXECUTE,
                });
            }
        }

        for n in 7..18 {
            if sizes[n] > 0 {
                // Data segments
                segments.push(Segment {
                    address: addresses[n],
                    size: sizes[n],
                    offset: offsets[n],
                    // TODO: We don't have nearly enough information to assume specific segments, so just set
                    // everything to RW. TODO: add signatures so we can confidently assume this is a MWCC
                    // binary and check extab/index?
                    permissions: Permissions::READ | Permissions::WRITE,
                })
            }
        }

        let bss_address = data.read_u32()?;
        let bss_size = data.read_u32()?;
        Self::calculate_unique_bss(&mut segments, bss_address, bss_size);

        // TODO: store this in a BTreeMap proper
        segments.sort_by(|a, b| a.address.cmp(&b.address));
        for segment in &segments {
            println!(
                "Segment {{ address: 0x{:08X}, size: 0x{:08X}, offset: 0x{:08X}, permissions: {:?} }}",
                segment.address, segment.size, segment.offset, segment.permissions
            );
        }

        Ok(segments)
    }

    fn calculate_unique_bss(segments: &mut Vec<Segment<u32>>, bss_address: u32, bss_size: u32) {
        // If the file somehow doesn't have a bss section, we can skip this whole thing
        if bss_size == 0 {
            return;
        }

        // Modeling this bss overlay as a series of transitions through other memory regions
        let mut transitions = BTreeSet::new();
        let bss_end = bss_address.saturating_add(bss_size);

        // Add existing segment transitions
        for seg in segments.iter() {
            let seg_end = seg.address.saturating_add(seg.size);
            transitions.insert((seg.address, 1)); // 1 = start
            transitions.insert((seg_end, -1)); // -1 = end
        }

        // Add BSS boundaries if they don't already exist
        transitions.insert((bss_address, 0)); // 0 = BSS boundary
        transitions.insert((bss_end, 0));

        let mut in_existing = 0i32;
        let mut last_point = None;

        for &(point, transition) in transitions.iter() {
            // If we have a valid previous point and we're in BSS range
            if let Some(start) = last_point {
                if point > start && in_existing == 0 && start >= bss_address && point <= bss_end {
                    segments.push(Segment {
                        address: start,
                        size: point - start,
                        offset: 0,
                        permissions: Permissions::READ | Permissions::WRITE | Permissions::UNINITIALIZED,
                    });
                }
            }

            // Update segment counter before processing next point
            in_existing += transition;
            last_point = Some(point);
        }
    }
}

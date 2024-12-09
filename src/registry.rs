use core::ops::Range;
use std::collections::BTreeMap;

// TODO: make this less stupid
#[derive(Clone, Debug)]
pub enum TypeInfo {
    Function { name: String, is_extern: bool },
    Integer { bits: u32, signed: bool },
    Struct { name: String, size: u64 },
    Union { name: String, size: u64 },
    Array { element_type: Box<TypeInfo>, count: u64 },
}

/// Designed with quickly fetching all types for a given address in mind.
#[derive(Debug, Default)]
pub struct TypeRegistry {
    lookup: BTreeMap<u64, Vec<(u64, TypeInfo)>>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self { lookup: BTreeMap::new() }
    }

    pub fn insert(&mut self, range: Range<u64>, type_info: TypeInfo) {
        self.lookup.entry(range.start).or_insert_with(Vec::new).push((range.end, type_info));
    }

    pub fn get_at_address(&self, address: u64) -> Vec<&TypeInfo> {
        let mut results = Vec::new();
        // Get all types whose start could possibly overlap with the address we're trying to look up.
        for (_start, ranges) in self.lookup.range(..=address) {
            // Iterate all types we've found and check which ones may overlap with the current adddress.
            for (end, type_info) in ranges {
                if address < *end {
                    results.push(type_info);
                }
            }
        }
        results
    }
}

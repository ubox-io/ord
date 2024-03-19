pub(super) type RuneEventEntryValue = (
  u128,                   // burned
  u8,                     // divisibility
  (u128, u128),           // etching
  Option<crate::index::entry::MintEntryValue>, // mint parameters
  u64,                    // mints
  u64,                    // number
  u128,                   // rune
  u32,                    // spacers
  u128,                   // supply
  Option<char>,           // symbol
  u32,                    // timestamp
);

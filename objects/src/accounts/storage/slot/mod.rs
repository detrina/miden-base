use vm_core::{
    utils::{ByteReader, ByteWriter, Deserializable, Serializable},
    EMPTY_WORD, ZERO,
};
use vm_processor::DeserializationError;

use super::{map::EMPTY_STORAGE_MAP_ROOT, Felt, StorageMap, Word};

mod r#type;
pub use r#type::StorageSlotType;

// STORAGE SLOT
// ================================================================================================

/// An object that represents the type of a storage slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageSlot {
    Value(Word),
    Map(StorageMap),
}

impl StorageSlot {
    /// The number of field elements needed to represent a [StorageSlot] in kernel memory.
    pub const NUM_ELEMENTS_PER_STORAGE_SLOT: usize = 8;

    /// Returns true if this storage slot has a value equal the default of it's type
    pub fn is_default(&self) -> bool {
        match self {
            StorageSlot::Value(value) => *value == EMPTY_WORD,
            StorageSlot::Map(map) => map.root() == EMPTY_STORAGE_MAP_ROOT,
        }
    }

    /// Returns the empty [Word] for a storage slot of this type
    pub fn default_word(&self) -> Word {
        match self {
            StorageSlot::Value(_) => EMPTY_WORD,
            StorageSlot::Map(_) => EMPTY_STORAGE_MAP_ROOT.into(),
        }
    }

    /// Returns a [`StorageSlot::Value`] with an empty word.
    pub fn empty_value() -> Self {
        StorageSlot::Value(EMPTY_WORD)
    }

    /// Returns an empty [`StorageSlot::Map`].
    pub fn empty_map() -> Self {
        StorageSlot::Map(StorageMap::new())
    }

    /// Returns this storage slot as field elements
    ///
    /// This is done by converting this storage slot into 8 field elements as follows:
    /// ```text
    /// [SLOT_VALUE, slot_type, 0, 0, 0]
    /// ```
    pub fn as_elements(&self) -> [Felt; Self::NUM_ELEMENTS_PER_STORAGE_SLOT] {
        let mut elements = [ZERO; 8];
        elements[0..4].copy_from_slice(&self.value());
        elements[4..8].copy_from_slice(&self.slot_type().as_word());
        elements
    }

    /// Returns this storage slot value as a [Word]
    ///
    /// Returns:
    /// - For [StorageSlot::Value] the value
    /// - For [StorageSlot::Map] the root of the [StorageMap]
    pub fn value(&self) -> Word {
        match self {
            Self::Value(value) => *value,
            Self::Map(map) => map.root().into(),
        }
    }

    /// Returns the type of this storage slot
    pub fn slot_type(&self) -> StorageSlotType {
        match self {
            StorageSlot::Value(_) => StorageSlotType::Value,
            StorageSlot::Map(_) => StorageSlotType::Map,
        }
    }
}

// SERIALIZATION
// ================================================================================================

impl Serializable for StorageSlot {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.slot_type());

        match self {
            Self::Value(value) => target.write(value),
            Self::Map(map) => target.write(map),
        }
    }

    fn get_size_hint(&self) -> usize {
        let mut size = self.slot_type().get_size_hint();

        size += match self {
            StorageSlot::Value(word) => word.get_size_hint(),
            StorageSlot::Map(storage_map) => storage_map.get_size_hint(),
        };

        size
    }
}

impl Deserializable for StorageSlot {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let storage_slot_type = source.read::<StorageSlotType>()?;

        match storage_slot_type {
            StorageSlotType::Value => {
                let word = source.read::<Word>()?;
                Ok(StorageSlot::Value(word))
            },
            StorageSlotType::Map => {
                let map = source.read::<StorageMap>()?;
                Ok(StorageSlot::Map(map))
            },
        }
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use vm_core::utils::{Deserializable, Serializable};

    use crate::accounts::AccountStorage;

    #[test]
    fn test_serde_account_storage_slot() {
        let storage = AccountStorage::mock();
        let serialized = storage.to_bytes();
        let deserialized = AccountStorage::read_from_bytes(&serialized).unwrap();
        assert_eq!(deserialized, storage)
    }
}
//! Usmap properties

use enum_dispatch::enum_dispatch;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{fmt::Debug, hash::Hash};

use crate::error::Error;

use self::{
    array_property::UsmapArrayPropertyData, enum_property::UsmapEnumPropertyData,
    map_property::UsmapMapPropertyData, set_property::UsmapSetPropertyData,
    shallow_property::UsmapShallowPropertyData, struct_property::UsmapStructPropertyData,
};

use super::{usmap_reader::UsmapReader, usmap_writer::UsmapWriter};

pub mod array_property;
pub mod enum_property;
pub mod map_property;
pub mod set_property;
pub mod shallow_property;
pub mod struct_property;

/// Usmap property type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum EPropertyType {
    /// Byte
    ByteProperty,
    /// Boolean
    BoolProperty,
    /// Int
    IntProperty,
    /// Float
    FloatProperty,
    /// Object
    ObjectProperty,
    /// Name
    NameProperty,
    /// Delegate
    DelegateProperty,
    /// Double
    DoubleProperty,
    /// Array
    ArrayProperty,
    /// Struct
    StructProperty,
    /// String
    StrProperty,
    /// Text
    TextProperty,
    /// Interface
    InterfaceProperty,
    /// MulticastDelegate
    MulticastDelegateProperty,
    /// WeakObject
    WeakObjectProperty, //
    /// LazyObject
    LazyObjectProperty, // When deserialized, these 3 properties will be SoftObjects
    /// AssetObject
    AssetObjectProperty, //
    /// SoftObject
    SoftObjectProperty,
    /// UInt64
    UInt64Property,
    /// UInt32
    UInt32Property,
    /// UInt16
    UInt16Property,
    /// Int64
    Int64Property,
    /// Int16
    Int16Property,
    /// Int8
    Int8Property,
    /// Map
    MapProperty,
    /// Set
    SetProperty,
    /// Enum
    EnumProperty,
    /// FieldPath
    FieldPathProperty,

    /// Unknown
    Unknown = 0xFF,
}

/// This must be implemented for all UsmapPropertyDatas
#[enum_dispatch]
pub trait UsmapPropertyDataTrait: Debug + Hash + Clone + PartialEq + Eq {
    /// Write `UsmapPropertyData` to an asset
    fn write<Writer: UsmapWriter>(&self, writer: &mut Writer) -> Result<usize, Error>;
}

/// UsmapPropertyData
#[enum_dispatch(UsmapPropertyDataTrait)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsmapPropertyData {
    /// Enum
    UsmapEnumPropertyData,
    /// Struct
    UsmapStructPropertyData,
    /// Set
    UsmapSetPropertyData,
    /// Array
    UsmapArrayPropertyData,
    /// Map
    UsmapMapPropertyData,

    /// Shallow
    UsmapShallowPropertyData,
}

impl UsmapPropertyData {
    /// Read an `UsmapPropertyData` from an asset
    pub fn new<Reader: UsmapReader>(asset: &mut Reader) -> Result<Self, Error> {
        let prop_type: EPropertyType = EPropertyType::try_from(asset.read_u8()?)?;

        let res: UsmapPropertyData = match prop_type {
            EPropertyType::ArrayProperty => UsmapArrayPropertyData::new(asset)?.into(),
            EPropertyType::StructProperty => UsmapStructPropertyData::new(asset)?.into(),
            EPropertyType::MapProperty => UsmapMapPropertyData::new(asset)?.into(),
            EPropertyType::SetProperty => UsmapSetPropertyData::new(asset)?.into(),
            EPropertyType::EnumProperty => UsmapEnumPropertyData::new(asset)?.into(),
            _ => UsmapShallowPropertyData {
                property_type: prop_type,
            }
            .into(),
        };

        Ok(res)
    }
}

/// UsmapProperty
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UsmapProperty {
    /// Name
    pub name: String,
    /// Schema index
    pub schema_index: u16,
    /// Array size
    pub array_size: u8,
    /// Property data
    pub property_data: UsmapPropertyData,
}

impl UsmapProperty {
    /// Read an `UsmapProperty` from an asset
    pub fn new<Reader: UsmapReader>(asset: &mut Reader) -> Result<Self, Error> {
        let schema_index = asset.read_u16()?;
        let array_size = asset.read_u8()?;
        let name = asset.read_name()?;

        let property_data = UsmapPropertyData::new(asset)?;
        Ok(UsmapProperty {
            name,
            schema_index,
            array_size,
            property_data,
        })
    }
}

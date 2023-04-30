//! Asset registry NameTableWriter
use std::io::{self, SeekFrom};

use byteorder::LittleEndian;

use crate::asset::name_map::NameMap;
use crate::containers::indexed_map::IndexedMap;
use crate::containers::shared_resource::SharedResource;
use crate::custom_version::{CustomVersion, CustomVersionTrait};
use crate::engine_version::EngineVersion;
use crate::error::{Error, FNameError};
use crate::object_version::{ObjectVersion, ObjectVersionUE5};
use crate::properties::Property;
use crate::reader::{archive_trait::ArchiveTrait, archive_writer::ArchiveWriter};
use crate::types::{FName, PackageIndex};
use crate::unversioned::header::UnversionedHeader;
use crate::Import;

/// Used to write NameTable entries by modifying the behavior
/// of some of the value write methods.
pub struct NameTableWriter<'writer, Writer: ArchiveWriter> {
    /// Writer
    writer: &'writer mut Writer,
    /// Name map
    name_map: SharedResource<NameMap>,
}

impl<'writer, Writer: ArchiveWriter> NameTableWriter<'writer, Writer> {
    /// Create a new `NameTableWriter` instance from another `Writer` and a name map
    pub fn new(writer: &'writer mut Writer, name_map: SharedResource<NameMap>) -> Self {
        NameTableWriter { writer, name_map }
    }
}

impl<'writer, Writer: ArchiveWriter> ArchiveTrait for NameTableWriter<'writer, Writer> {
    fn get_custom_version<T>(&self) -> CustomVersion
    where
        T: CustomVersionTrait + Into<i32>,
    {
        self.writer.get_custom_version::<T>()
    }

    fn position(&mut self) -> u64 {
        self.writer.position()
    }

    fn set_position(&mut self, pos: u64) {
        self.writer.set_position(pos)
    }

    fn seek(&mut self, style: SeekFrom) -> io::Result<u64> {
        self.writer.seek(style)
    }

    fn get_name_map(&self) -> SharedResource<NameMap> {
        self.name_map.clone()
    }

    fn get_name_reference(&self, index: i32) -> String {
        self.name_map.get_ref().get_name_reference(index)
    }

    fn get_array_struct_type_override(&self) -> &IndexedMap<String, String> {
        self.writer.get_array_struct_type_override()
    }

    fn get_map_key_override(&self) -> &IndexedMap<String, String> {
        self.writer.get_map_key_override()
    }

    fn get_map_value_override(&self) -> &IndexedMap<String, String> {
        self.writer.get_map_value_override()
    }

    fn get_parent_class(&self) -> Option<crate::ParentClassInfo> {
        self.writer.get_parent_class()
    }

    fn get_parent_class_cached(&mut self) -> Option<&crate::ParentClassInfo> {
        self.writer.get_parent_class_cached()
    }

    fn get_engine_version(&self) -> EngineVersion {
        self.writer.get_engine_version()
    }

    fn get_object_version(&self) -> ObjectVersion {
        self.writer.get_object_version()
    }

    fn get_object_version_ue5(&self) -> ObjectVersionUE5 {
        self.writer.get_object_version_ue5()
    }

    fn get_import(&self, index: PackageIndex) -> Option<&Import> {
        self.writer.get_import(index)
    }

    fn get_export_class_type(&self, index: PackageIndex) -> Option<FName> {
        self.writer.get_export_class_type(index)
    }

    fn add_fname(&mut self, value: &str) -> FName {
        self.writer.add_fname(value)
    }

    fn add_fname_with_number(&mut self, value: &str, number: i32) -> FName {
        self.writer.add_fname_with_number(value, number)
    }

    fn get_mappings(&self) -> Option<&crate::unversioned::Usmap> {
        self.writer.get_mappings()
    }

    fn has_unversioned_properties(&self) -> bool {
        self.writer.has_unversioned_properties()
    }
}

impl<'writer, Writer: ArchiveWriter> ArchiveWriter for NameTableWriter<'writer, Writer> {
    fn write_property_guid(
        &mut self,
        guid: &Option<crate::types::Guid>,
    ) -> Result<(), crate::error::Error> {
        self.writer.write_property_guid(guid)
    }

    fn write_fname(&mut self, fname: &FName) -> Result<(), crate::error::Error> {
        match fname {
            FName::Backed {
                index,
                number,
                name_map: _,
            } => {
                self.writer.write_i32::<LittleEndian>(*index)?;
                self.writer.write_i32::<LittleEndian>(*number)?;
                Ok(())
            }
            FName::Dummy { value, number } => {
                Err(FNameError::dummy_serialize(value, *number).into())
            }
        }
    }

    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.writer.write_u8(value)
    }

    fn write_i8(&mut self, value: i8) -> io::Result<()> {
        self.writer.write_i8(value)
    }

    fn write_u16<T: byteorder::ByteOrder>(&mut self, value: u16) -> io::Result<()> {
        self.writer.write_u16::<T>(value)
    }

    fn write_i16<T: byteorder::ByteOrder>(&mut self, value: i16) -> io::Result<()> {
        self.writer.write_i16::<T>(value)
    }

    fn write_u32<T: byteorder::ByteOrder>(&mut self, value: u32) -> io::Result<()> {
        self.writer.write_u32::<T>(value)
    }

    fn write_i32<T: byteorder::ByteOrder>(&mut self, value: i32) -> io::Result<()> {
        self.writer.write_i32::<T>(value)
    }

    fn write_u64<T: byteorder::ByteOrder>(&mut self, value: u64) -> io::Result<()> {
        self.writer.write_u64::<T>(value)
    }

    fn write_i64<T: byteorder::ByteOrder>(&mut self, value: i64) -> io::Result<()> {
        self.writer.write_i64::<T>(value)
    }

    fn write_f32<T: byteorder::ByteOrder>(&mut self, value: f32) -> io::Result<()> {
        self.writer.write_f32::<T>(value)
    }

    fn write_f64<T: byteorder::ByteOrder>(&mut self, value: f64) -> io::Result<()> {
        self.writer.write_f64::<T>(value)
    }

    fn write_fstring(&mut self, value: Option<&str>) -> Result<usize, Error> {
        self.writer.write_fstring(value)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }

    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.writer.write_bool(value)
    }

    fn generate_unversioned_header(
        &mut self,
        properties: &[Property],
        parent_name: &FName,
    ) -> Result<Option<(UnversionedHeader, Vec<Property>)>, Error> {
        self.writer
            .generate_unversioned_header(properties, parent_name)
    }
}

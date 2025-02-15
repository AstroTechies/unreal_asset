//! Usmap file writer

use std::io::{Seek, Write};

use crate::{
    containers::{indexed_map::IndexedMap, name_map::NameMap, shared_resource::SharedResource},
    custom_version::{CustomVersion, CustomVersionTrait},
    engine_version::EngineVersion,
    error::Error,
    object_version::{ObjectVersion, ObjectVersionUE5},
    passthrough_archive_writer,
    reader::{
        archive_trait::{ArchiveTrait, ArchiveType},
        archive_writer::ArchiveWriter,
    },
    types::{FName, PackageIndex},
};

use super::Usmap;

/// Usmap file writer
pub struct UsmapWriter<'parent_writer, 'asset, W: ArchiveWriter<PackageIndex>> {
    /// Parent writer
    parent_writer: &'parent_writer mut W,
    /// Name map
    _name_map: &'asset [String],
    /// Custom versions
    custom_versions: &'asset [CustomVersion],
}

impl<W: ArchiveWriter<PackageIndex>>
    UsmapWriter<'_, '_, W>
{
    /// Write a name to this archive
    pub fn write_name(&mut self, _: &str) -> Result<usize, Error> {
        todo!()
    }
}

impl<W: ArchiveWriter<PackageIndex>> ArchiveTrait<PackageIndex>
    for UsmapWriter<'_, '_, W>
{
    fn get_archive_type(&self) -> ArchiveType {
        ArchiveType::Usmap
    }

    fn get_custom_version<T>(&self) -> CustomVersion
    where
        T: CustomVersionTrait + Into<i32>,
    {
        self.custom_versions
            .iter()
            .find(|e| e.guid == T::GUID)
            .cloned()
            .unwrap_or_else(|| CustomVersion::new(T::GUID, 0))
    }

    fn has_unversioned_properties(&self) -> bool {
        false
    }

    fn use_event_driven_loader(&self) -> bool {
        false
    }

    fn position(&mut self) -> u64 {
        self.parent_writer.position()
    }

    fn get_name_map(&self) -> SharedResource<NameMap> {
        self.parent_writer.get_name_map()
    }

    fn get_array_struct_type_override(&self) -> &IndexedMap<String, String> {
        self.parent_writer.get_array_struct_type_override()
    }

    fn get_map_key_override(&self) -> &IndexedMap<String, String> {
        self.parent_writer.get_map_key_override()
    }

    fn get_map_value_override(&self) -> &IndexedMap<String, String> {
        self.parent_writer.get_map_value_override()
    }

    fn get_engine_version(&self) -> EngineVersion {
        self.parent_writer.get_engine_version()
    }

    fn get_object_version(&self) -> ObjectVersion {
        self.parent_writer.get_object_version()
    }

    fn get_object_version_ue5(&self) -> ObjectVersionUE5 {
        self.parent_writer.get_object_version_ue5()
    }

    fn get_mappings(&self) -> Option<&Usmap> {
        None
    }

    fn get_parent_class_export_name(&self) -> Option<FName> {
        self.parent_writer.get_parent_class_export_name()
    }

    fn get_object_name(&self, index: PackageIndex) -> Option<FName> {
        self.parent_writer.get_object_name(index)
    }

    fn get_object_name_packageindex(&self, index: PackageIndex) -> Option<FName> {
        self.parent_writer.get_object_name_packageindex(index)
    }
}

impl<W: ArchiveWriter<PackageIndex>> ArchiveWriter<PackageIndex>
    for UsmapWriter<'_, '_, W>
{
    passthrough_archive_writer!(parent_writer);
}

impl<W: ArchiveWriter<PackageIndex>> Write
    for UsmapWriter<'_, '_, W>
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.parent_writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.parent_writer.flush()
    }
}

impl<W: ArchiveWriter<PackageIndex>> Seek
    for UsmapWriter<'_, '_, W>
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.parent_writer.seek(pos)
    }
}

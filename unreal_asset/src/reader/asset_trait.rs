use std::io::{self, SeekFrom};

use crate::containers::indexed_map::IndexedMap;
use crate::custom_version::{CustomVersion, CustomVersionTrait};
use crate::engine_version::EngineVersion;
use crate::object_version::{ObjectVersion, ObjectVersionUE5};
use crate::unreal_types::{FName, PackageIndex};
use crate::unversioned::Usmap;
use crate::Import;

/// A trait that allows accessing data about the archive that is currently being read
pub trait AssetTrait {
    fn get_custom_version<T>(&self) -> CustomVersion
    where
        T: CustomVersionTrait + Into<i32>;
    fn position(&self) -> u64;
    fn set_position(&mut self, pos: u64);
    fn seek(&mut self, style: SeekFrom) -> io::Result<u64>;

    fn add_fname(&mut self, value: &str) -> FName;
    fn add_fname_with_number(&mut self, value: &str, number: i32) -> FName;

    fn get_name_map_index_list(&self) -> &[String];
    fn get_name_reference(&self, index: i32) -> String;

    fn get_map_key_override(&self) -> &IndexedMap<String, String>;
    fn get_map_value_override(&self) -> &IndexedMap<String, String>;

    fn get_engine_version(&self) -> EngineVersion;
    fn get_object_version(&self) -> ObjectVersion;
    fn get_object_version_ue5(&self) -> ObjectVersionUE5;

    fn get_mappings(&self) -> Option<&Usmap>;

    fn get_import(&self, index: PackageIndex) -> Option<&Import>;
    fn get_export_class_type(&self, index: PackageIndex) -> Option<FName>;
}

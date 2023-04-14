//! Normal export

use crate::error::Error;
use crate::exports::{base_export::BaseExport, ExportBaseTrait, ExportNormalTrait, ExportTrait};
use crate::properties::Property;
use crate::reader::{asset_reader::AssetReader, asset_writer::AssetWriter};
use crate::types::FName;

/// Normal export
///
/// This export is usually the base export for all other exports
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NormalExport {
    /// Base export
    pub base_export: BaseExport,
    /// Extra data
    pub extras: Vec<u8>,
    /// Properties
    pub properties: Vec<Property>,
}

impl ExportNormalTrait for NormalExport {
    fn get_normal_export(&'_ self) -> Option<&'_ NormalExport> {
        Some(self)
    }

    fn get_normal_export_mut(&'_ mut self) -> Option<&'_ mut NormalExport> {
        Some(self)
    }
}

impl ExportBaseTrait for NormalExport {
    fn get_base_export(&'_ self) -> &'_ BaseExport {
        &self.base_export
    }

    fn get_base_export_mut(&'_ mut self) -> &'_ mut BaseExport {
        &mut self.base_export
    }
}

impl NormalExport {
    /// Read a `NormalExport` from an asset
    pub fn from_base<Reader: AssetReader>(
        base: &BaseExport,
        asset: &mut Reader,
    ) -> Result<Self, Error> {
        let mut properties = Vec::new();

        let parent_name = asset
            .get_parent_class_cached()
            .map(|e| e.parent_class_export_name.clone());

        while let Some(e) = Property::new(asset, parent_name.as_ref(), true)? {
            properties.push(e);
        }

        Ok(NormalExport {
            base_export: base.clone(),
            extras: Vec::new(),

            properties,
        })
    }
}

impl ExportTrait for NormalExport {
    fn write<Writer: AssetWriter>(&self, asset: &mut Writer) -> Result<(), Error> {
        for entry in &self.properties {
            Property::write(entry, asset, true)?;
        }
        asset.write_fname(&FName::from_slice("None"))?;
        Ok(())
    }
}

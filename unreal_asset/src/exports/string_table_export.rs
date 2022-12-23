use byteorder::LittleEndian;

use crate::containers::indexed_map::IndexedMap;
use crate::error::Error;
use crate::exports::{
    base_export::BaseExport, normal_export::NormalExport, ExportBaseTrait, ExportNormalTrait,
    ExportTrait,
};
use crate::implement_get;
use crate::reader::{asset_reader::AssetReader, asset_writer::AssetWriter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringTableExport {
    normal_export: NormalExport,

    pub namespace: Option<String>,
    pub table: IndexedMap<String, String>,
}

implement_get!(StringTableExport);

impl StringTableExport {
    pub fn from_base<Reader: AssetReader>(
        base: &BaseExport,
        asset: &mut Reader,
    ) -> Result<Self, Error> {
        let normal_export = NormalExport::from_base(base, asset)?;
        asset.read_i32::<LittleEndian>()?;

        let namespace = asset.read_string()?;

        let mut table = IndexedMap::new();
        let num_entries = asset.read_i32::<LittleEndian>()?;
        for _ in 0..num_entries {
            table.insert(
                asset
                    .read_string()?
                    .ok_or_else(|| Error::no_data("StringTable key is None".to_string()))?,
                asset
                    .read_string()?
                    .ok_or_else(|| Error::no_data("StringTable value is None".to_string()))?,
            );
        }

        Ok(StringTableExport {
            normal_export,
            namespace,
            table,
        })
    }
}

impl ExportTrait for StringTableExport {
    fn write<Writer: AssetWriter>(&self, asset: &mut Writer) -> Result<(), Error> {
        self.normal_export.write(asset)?;
        asset.write_i32::<LittleEndian>(0)?;

        asset.write_string(&self.namespace)?;
        asset.write_i32::<LittleEndian>(self.table.len() as i32)?;
        for (_, key, value) in &self.table {
            asset.write_string(&Some(key.clone()))?;
            asset.write_string(&Some(value.clone()))?;
        }
        Ok(())
    }
}

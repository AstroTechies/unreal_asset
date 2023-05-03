//! Sampler properties

use std::mem::size_of;

use byteorder::LittleEndian;
use ordered_float::OrderedFloat;
use unreal_asset_proc_macro::FNameContainer;

use crate::error::Error;
use crate::impl_property_data_trait;
use crate::optional_guid;
use crate::optional_guid_write;
use crate::properties::PropertyTrait;
use crate::reader::{archive_reader::ArchiveReader, archive_writer::ArchiveWriter};
use crate::types::{fname::FName, Guid};
use crate::unversioned::ancestry::Ancestry;

/// Weighted random sampler property
#[derive(FNameContainer, Debug, Hash, Clone, PartialEq, Eq)]
pub struct WeightedRandomSamplerProperty {
    /// Name
    pub name: FName,
    /// Property ancestry
    pub ancestry: Ancestry,
    /// Property guid
    pub property_guid: Option<Guid>,
    /// Property duplication index
    pub duplication_index: i32,
    /// Probabilities
    pub prob: Vec<OrderedFloat<f32>>,
    /// Alias
    pub alias: Vec<i32>,
    /// Total sampler weight
    pub total_weight: OrderedFloat<f32>,
}
impl_property_data_trait!(WeightedRandomSamplerProperty);

/// Skeletal mesh area weighted triangle sampler
#[derive(FNameContainer, Debug, Hash, Clone, PartialEq, Eq)]
pub struct SkeletalMeshAreaWeightedTriangleSampler {
    /// Name
    pub name: FName,
    /// Property ancestry
    pub ancestry: Ancestry,
    /// Property guid
    pub property_guid: Option<Guid>,
    /// Property duplication index
    pub duplication_index: i32,
    /// Probabilities
    pub prob: Vec<OrderedFloat<f32>>,
    /// Alias
    pub alias: Vec<i32>,
    /// Total sampler weight
    pub total_weight: OrderedFloat<f32>,
}
impl_property_data_trait!(SkeletalMeshAreaWeightedTriangleSampler);

/// Skeleetal mesh sampling lod built data property
#[derive(FNameContainer, Debug, Hash, Clone, PartialEq, Eq)]
pub struct SkeletalMeshSamplingLODBuiltDataProperty {
    /// Name
    pub name: FName,
    /// Property ancestry
    pub ancestry: Ancestry,
    /// Property guid
    pub property_guid: Option<Guid>,
    /// Property duplication index
    pub duplication_index: i32,
    /// Sampler
    pub sampler_property: WeightedRandomSamplerProperty,
}
impl_property_data_trait!(SkeletalMeshSamplingLODBuiltDataProperty);

impl WeightedRandomSamplerProperty {
    /// Read a `WeightedRandomSamplerProperty` from an asset
    pub fn new<Reader: ArchiveReader>(
        asset: &mut Reader,
        name: FName,
        ancestry: Ancestry,
        include_header: bool,
        _length: i64,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let property_guid = optional_guid!(asset, include_header);

        let size = asset.read_i32::<LittleEndian>()?;
        let mut prob = Vec::with_capacity(size as usize);
        for _i in 0..size as usize {
            prob.push(OrderedFloat(asset.read_f32::<LittleEndian>()?));
        }

        let size = asset.read_i32::<LittleEndian>()?;
        let mut alias = Vec::with_capacity(size as usize);
        for _i in 0..size as usize {
            alias.push(asset.read_i32::<LittleEndian>()?);
        }

        let total_weight = OrderedFloat(asset.read_f32::<LittleEndian>()?);

        Ok(WeightedRandomSamplerProperty {
            name,
            ancestry,
            property_guid,
            duplication_index,
            prob,
            alias,
            total_weight,
        })
    }
}

impl PropertyTrait for WeightedRandomSamplerProperty {
    fn write<Writer: ArchiveWriter>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        optional_guid_write!(self, asset, include_header);
        asset.write_i32::<LittleEndian>(self.prob.len() as i32)?;
        for entry in &self.prob {
            asset.write_f32::<LittleEndian>(entry.0)?;
        }

        asset.write_i32::<LittleEndian>(self.alias.len() as i32)?;
        for entry in &self.alias {
            asset.write_i32::<LittleEndian>(*entry)?;
        }

        asset.write_f32::<LittleEndian>(self.total_weight.0)?;
        Ok(size_of::<i32>()
            + size_of::<f32>() * self.prob.len()
            + size_of::<i32>()
            + size_of::<i32>() * self.alias.len()
            + size_of::<f32>())
    }
}

impl SkeletalMeshAreaWeightedTriangleSampler {
    /// Read a `SkeletalMeshAreaWeightedTriangleSampler` from an asset
    pub fn new<Reader: ArchiveReader>(
        asset: &mut Reader,
        name: FName,
        ancestry: Ancestry,
        include_header: bool,
        _length: i64,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let property_guid = optional_guid!(asset, include_header);

        let size = asset.read_i32::<LittleEndian>()?;
        let mut prob = Vec::with_capacity(size as usize);
        for _i in 0..size as usize {
            prob.push(OrderedFloat(asset.read_f32::<LittleEndian>()?));
        }

        let size = asset.read_i32::<LittleEndian>()?;
        let mut alias = Vec::with_capacity(size as usize);
        for _i in 0..size as usize {
            alias.push(asset.read_i32::<LittleEndian>()?);
        }

        let total_weight = OrderedFloat(asset.read_f32::<LittleEndian>()?);

        Ok(SkeletalMeshAreaWeightedTriangleSampler {
            name,
            ancestry,
            property_guid,
            duplication_index,
            prob,
            alias,
            total_weight,
        })
    }
}

impl PropertyTrait for SkeletalMeshAreaWeightedTriangleSampler {
    fn write<Writer: ArchiveWriter>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        optional_guid_write!(self, asset, include_header);
        asset.write_i32::<LittleEndian>(self.prob.len() as i32)?;
        for entry in &self.prob {
            asset.write_f32::<LittleEndian>(entry.0)?;
        }

        asset.write_i32::<LittleEndian>(self.alias.len() as i32)?;
        for entry in &self.alias {
            asset.write_i32::<LittleEndian>(*entry)?;
        }

        asset.write_f32::<LittleEndian>(self.total_weight.0)?;
        Ok(size_of::<i32>()
            + size_of::<f32>() * self.prob.len()
            + size_of::<i32>()
            + size_of::<i32>() * self.alias.len()
            + size_of::<f32>())
    }
}

impl SkeletalMeshSamplingLODBuiltDataProperty {
    /// Read a `SkeletalMeshSamplingLODBuiltDataProperty` from an asset
    pub fn new<Reader: ArchiveReader>(
        asset: &mut Reader,
        name: FName,
        ancestry: Ancestry,
        include_header: bool,
        _length: i64,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let property_guid = optional_guid!(asset, include_header);
        let sampler_property = WeightedRandomSamplerProperty::new(
            asset,
            name.clone(),
            ancestry.with_parent(name.clone()),
            false,
            0,
            0,
        )?;

        Ok(SkeletalMeshSamplingLODBuiltDataProperty {
            name,
            ancestry,
            property_guid,
            duplication_index,
            sampler_property,
        })
    }
}

impl PropertyTrait for SkeletalMeshSamplingLODBuiltDataProperty {
    fn write<Writer: ArchiveWriter>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        optional_guid_write!(self, asset, include_header);
        self.sampler_property.write(asset, false)
    }
}

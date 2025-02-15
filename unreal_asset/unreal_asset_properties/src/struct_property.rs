//! Struct property

use crate::property_prelude::*;

/// Struct property
#[derive(FNameContainer, Debug, Hash, Clone, Default, PartialEq, Eq)]
pub struct StructProperty {
    /// Name
    pub name: FName,
    /// Property ancestry
    pub ancestry: Ancestry,
    /// Struct type
    pub struct_type: Option<FName>,
    /// Struct guid
    pub struct_guid: Option<Guid>,
    /// Property guid
    pub property_guid: Option<Guid>,
    /// Property duplication index
    pub duplication_index: i32,
    /// Should serialize None
    pub serialize_none: bool,
    /// Struct variables
    pub value: Vec<Property>,
}
impl_property_data_trait!(StructProperty);

impl StructProperty {
    /// Create a dummy `StructProperty`
    pub fn dummy(
        name: FName,
        ancestry: Ancestry,
        struct_type: FName,
        struct_guid: Option<Guid>,
    ) -> Self {
        StructProperty {
            name,
            ancestry,
            struct_type: Some(struct_type),
            struct_guid,
            property_guid: None,
            duplication_index: 0,
            serialize_none: true,
            value: Vec::new(),
        }
    }

    /// Read a `StructProperty` from an asset
    pub fn new<Reader: ArchiveReader<impl PackageIndexTrait>>(
        asset: &mut Reader,
        name: FName,
        ancestry: Ancestry,
        include_header: bool,
        length: i64,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let mut struct_type = None;
        let mut struct_guid = None;
        let mut property_guid = None;

        if include_header && !asset.has_unversioned_properties() {
            struct_type = Some(asset.read_fname()?);
            if asset.get_object_version() >= ObjectVersion::VER_UE4_STRUCT_GUID_IN_PROPERTY_TAG {
                struct_guid = Some(asset.read_guid()?);
            }
            property_guid = asset.read_property_guid()?;
        }

        StructProperty::custom_header(
            asset,
            name,
            ancestry,
            length,
            duplication_index,
            struct_type,
            struct_guid,
            property_guid,
        )
    }

    /// Read a `StructProperty` with custom header values set
    #[allow(clippy::too_many_arguments)]
    pub fn custom_header<Reader: ArchiveReader<impl PackageIndexTrait>>(
        asset: &mut Reader,
        name: FName,
        ancestry: Ancestry,
        length: i64,
        duplication_index: i32,
        mut struct_type: Option<FName>,
        struct_guid: Option<Guid>,
        property_guid: Option<Guid>,
    ) -> Result<Self, Error> {
        if let Some(struct_mapping) = asset
            .get_mappings()
            .and_then(|e| e.get_property(&name, &ancestry))
            .and_then(|e| cast!(UsmapPropertyData, UsmapStructPropertyData, &e.property_data))
        {
            if struct_type.as_ref().map(|e| e == "Generic").unwrap_or(true) {
                struct_type = Some(FName::new_dummy(struct_mapping.struct_type.clone(), 0));
            }
        }

        if asset.has_unversioned_properties() && struct_type.is_none() {
            return name.get_content(|name| Err(PropertyError::no_type(name, &ancestry).into()));
        }

        let mut custom_serialization = match struct_type {
            Some(ref e) => e.get_content(Property::has_custom_serialization),
            None => false,
        };

        struct_type
            .as_ref()
            .unwrap_or(&FName::from_slice(""))
            .get_content(|ty| {
                match ty {
                    "FloatRange" => {
                        // FloatRange is a special case; it can either be manually serialized as two floats (TRange<float>) or as a regular struct (FFloatRange), but the first is overridden to use the same name as the second
                        // The best solution is to just check and see if the next bit is an FName or not

                        let name_map_index = asset.read_i32::<LE>()?;
                        asset.seek(SeekFrom::Current(-(size_of::<u32>() as i64)))?;

                        let is_lower_bound = match name_map_index >= 0
                            && name_map_index
                                < asset
                                    .get_name_map()
                                    .get_ref()
                                    .get_name_map_index_list()
                                    .len() as i32
                        {
                            true => asset
                                .get_name_reference(name_map_index, |name| name == "LowerBound"),
                            false => false,
                        };
                        custom_serialization =
                            !(asset.has_unversioned_properties() || is_lower_bound);
                    }
                    "RichCurveKey"
                        if asset.get_object_version()
                            < ObjectVersion::VER_UE4_SERIALIZE_RICH_CURVE_KEY =>
                    {
                        custom_serialization = false;
                    }
                    "MovieSceneTrackIdentifier"
                        if asset.get_custom_version::<FEditorObjectVersion>().version
                            < FEditorObjectVersion::MovieSceneMetaDataSerialization as i32 =>
                    {
                        custom_serialization = false;
                    }
                    "MovieSceneFloatChannel" => {
                        if asset
                            .get_custom_version::<FSequencerObjectVersion>()
                            .version
                            < FSequencerObjectVersion::SerializeFloatChannelCompletely as i32
                            && asset
                                .get_custom_version::<FFortniteMainBranchObjectVersion>()
                                .version
                                < FFortniteMainBranchObjectVersion::SerializeFloatChannelShowCurve
                                    as i32
                        {
                            custom_serialization = false;
                        }
                    }
                    _ => {}
                };
                Ok::<(), Error>(())
            })?;

        if length == 0 {
            return Ok(StructProperty {
                name,
                ancestry,
                struct_type,
                struct_guid,
                property_guid,
                duplication_index,
                serialize_none: false,
                value: Vec::new(),
            });
        }

        if custom_serialization {
            let new_ancestry = ancestry.with_parent(name.clone());
            let property = Property::from_type(
                asset,
                struct_type.as_ref().unwrap(),
                name.clone(),
                new_ancestry,
                false,
                0,
                0,
                0,
                false,
            )?;
            let value = vec![property];

            Ok(StructProperty {
                name,
                ancestry,
                struct_type,
                struct_guid,
                property_guid,
                duplication_index,
                serialize_none: true,
                value,
            })
        } else {
            let mut values = Vec::new();
            let mut unversioned_header = UnversionedHeader::new(asset)?;
            let new_ancestry = ancestry.with_parent(struct_type.clone().unwrap());
            while let Some(property) = Property::new(
                asset,
                new_ancestry.clone(),
                unversioned_header.as_mut(),
                true,
            )? {
                values.push(property);
            }

            Ok(StructProperty {
                name,
                ancestry,
                struct_type,
                struct_guid,
                property_guid,
                duplication_index,
                serialize_none: true,
                value: values,
            })
        }
    }

    /// Write a `StructProperty` overriding struct type
    pub fn write_with_type<Writer: ArchiveWriter<impl PackageIndexTrait>>(
        &self,
        asset: &mut Writer,
        include_header: bool,
        struct_type: Option<FName>,
    ) -> Result<usize, Error> {
        if include_header {
            asset.write_fname(struct_type.as_ref().ok_or_else(PropertyError::headerless)?)?;
            if asset.get_object_version() >= ObjectVersion::VER_UE4_STRUCT_GUID_IN_PROPERTY_TAG {
                asset.write_guid(&self.struct_guid.ok_or_else(PropertyError::headerless)?)?;
            }
            asset.write_property_guid(self.property_guid.as_ref())?;
        }

        let mut has_custom_serialization = match struct_type {
            Some(ref e) => e.get_content(Property::has_custom_serialization),
            None => false,
        };

        if let Some(ref struct_type) = struct_type {
            if struct_type == "FloatRange" {
                has_custom_serialization = self.value.len() == 1
                    && cast!(Property, FloatRangeProperty, &self.value[0]).is_some();
            }

            if struct_type == "RichCurveKey"
                && asset.get_object_version() < ObjectVersion::VER_UE4_SERIALIZE_RICH_CURVE_KEY
            {
                has_custom_serialization = false;
            }

            if struct_type == "MovieSceneTrackIdentifier"
                && asset.get_custom_version::<FEditorObjectVersion>().version
                    < FEditorObjectVersion::MovieSceneMetaDataSerialization as i32
            {
                has_custom_serialization = false;
            }

            if struct_type == "MovieSceneFloatChannel"
                && asset
                    .get_custom_version::<FSequencerObjectVersion>()
                    .version
                    < FSequencerObjectVersion::SerializeFloatChannelCompletely as i32
                && asset
                    .get_custom_version::<FFortniteMainBranchObjectVersion>()
                    .version
                    < FFortniteMainBranchObjectVersion::SerializeFloatChannelShowCurve as i32
            {
                has_custom_serialization = false;
            }
        }

        if has_custom_serialization {
            if self.value.len() != 1 {
                return Err(PropertyError::invalid_struct(
                    struct_type
                        .unwrap_or_else(|| FName::from_slice("Generic"))
                        .get_content(|e| {
                            format!("Structs with type {} must have exactly 1 entry", e)
                        }),
                )
                .into());
            }
            self.value[0].write(asset, false)
        } else if self.value.is_empty() && !self.serialize_none {
            Ok(0)
        } else {
            let begin = asset.position();

            let (unversioned_header, sorted_properties) = match generate_unversioned_header(
                asset,
                &self.value,
                self.struct_type.as_ref().unwrap_or(&FName::default()),
            )? {
                Some((a, b)) => (Some(a), Some(b)),
                None => (None, None),
            };

            if let Some(unversioned_header) = unversioned_header {
                unversioned_header.write(asset)?;
            }

            let properties = sorted_properties.as_ref().unwrap_or(&self.value);
            for entry in properties.iter() {
                Property::write(entry, asset, true)?;
            }

            if !asset.has_unversioned_properties() {
                asset.write_fname(&asset.get_name_map().get_mut().add_fname("None"))?;
            }
            Ok((asset.position() - begin) as usize)
        }
    }
}

impl PropertyTrait for StructProperty {
    fn write<Writer: ArchiveWriter<impl PackageIndexTrait>>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        self.write_with_type(asset, include_header, self.struct_type.clone())
    }
}

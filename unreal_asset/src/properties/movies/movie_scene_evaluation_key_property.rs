//! Movie scene evaluation key property

use byteorder::LittleEndian;

use crate::{
    error::Error,
    impl_property_data_trait, optional_guid, optional_guid_write,
    properties::PropertyTrait,
    reader::{asset_reader::AssetReader, asset_writer::AssetWriter},
    types::{FName, Guid},
    unversioned::ancestry::Ancestry,
};

use super::{
    movie_scene_sequence_id_property::MovieSceneSequenceId,
    movie_scene_track_identifier_property::MovieSceneTrackIdentifier,
};

/// Movie scene evaluation key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovieSceneEvaluationKey {
    /// Movie sequence id
    pub sequence_id: MovieSceneSequenceId,
    /// Movie track identifier
    pub track_identifier: MovieSceneTrackIdentifier,
    /// Movie section index
    pub section_index: u32,
}

impl MovieSceneEvaluationKey {
    /// Read a `MovieSceneEvaluationKey` from an asset
    pub fn new<Reader: AssetReader>(asset: &mut Reader) -> Result<Self, Error> {
        let sequence_id = MovieSceneSequenceId::new(asset)?;
        let track_identifier = MovieSceneTrackIdentifier::new(asset)?;
        let section_index = asset.read_u32::<LittleEndian>()?;

        Ok(MovieSceneEvaluationKey {
            sequence_id,
            track_identifier,
            section_index,
        })
    }

    /// Write a `MovieSceneEvaluationKey` to an asset
    pub fn write<Writer: AssetWriter>(&self, asset: &mut Writer) -> Result<(), Error> {
        self.sequence_id.write(asset)?;
        self.track_identifier.write(asset)?;
        asset.write_u32::<LittleEndian>(self.section_index)?;

        Ok(())
    }
}

/// Movie scene evaluation key property
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovieSceneEvaluationKeyProperty {
    /// Name
    pub name: FName,
    /// Property ancestry
    pub ancestry: Ancestry,
    /// Property guid
    pub property_guid: Option<Guid>,
    /// Property duplication index
    pub duplication_index: i32,
    /// Value
    pub value: MovieSceneEvaluationKey,
}
impl_property_data_trait!(MovieSceneEvaluationKeyProperty);

impl MovieSceneEvaluationKeyProperty {
    /// Read a `MovieSceneEvaluationKeyProperty` from an asset
    pub fn new<Reader: AssetReader>(
        asset: &mut Reader,
        name: FName,
        ancestry: Ancestry,
        include_header: bool,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let property_guid = optional_guid!(asset, include_header);

        let value = MovieSceneEvaluationKey::new(asset)?;

        Ok(MovieSceneEvaluationKeyProperty {
            name,
            ancestry,
            property_guid,
            duplication_index,
            value,
        })
    }
}

impl PropertyTrait for MovieSceneEvaluationKeyProperty {
    fn write<Writer: AssetWriter>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        optional_guid_write!(self, asset, include_header);

        let begin = asset.position();

        self.value.write(asset)?;

        Ok((asset.position() - begin) as usize)
    }
}

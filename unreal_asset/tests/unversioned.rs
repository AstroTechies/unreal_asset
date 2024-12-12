use std::io::Cursor;

use unreal_asset::{engine_version::EngineVersion, Asset, Error};

mod shared;

macro_rules! assets_folder {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets/unversioned/")
    };
}

const TEST_ASSETS: [(&[u8], &[u8], &[u8]); 1] = [(
    include_bytes!(concat!(assets_folder!(), "MGPlayer.uasset")),
    include_bytes!(concat!(assets_folder!(), "MGPlayer.uexp")),
    include_bytes!(concat!(assets_folder!(), "MetroGravity.usmap")),
)];

#[test]
fn unversioned() -> Result<(), Error> {
    for (asset_data, bulk_data, mapping) in TEST_ASSETS {
        let mut parsed = Asset::new(
            Cursor::new(asset_data),
            Some(Cursor::new(bulk_data)),
            EngineVersion::VER_UE5_2,
            Some(unreal_asset::unversioned::Usmap::new(Cursor::new(mapping))?),
        )?;
        shared::verify_binary_equality(asset_data, Some(bulk_data), &mut parsed)?;
        shared::verify_all_exports_parsed(&parsed);
    }

    Ok(())
}

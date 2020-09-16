use atelier_core::AssetUuid;
use atelier_importer::{AsyncImporter, Error, ImportedAsset, ImporterValue, Result};
use futures_core::future::BoxFuture;
use futures_io::AsyncRead;
use futures_util::AsyncReadExt;
use image2::{color, ImageBuf};
use serde::{Deserialize, Serialize};
use type_uuid::*;

#[derive(TypeUuid, Serialize, Deserialize)]
#[uuid = "d4079e74-3ec9-4ebc-9b77-a87cafdfdada"]
pub enum Image {
    Rgb8(ImageBuf<u8, color::Rgb>),
    // ...
}

#[derive(TypeUuid, Serialize, Deserialize, Default)]
#[uuid = "3c8367c8-45fb-40bb-a229-00e5e9c3fc70"]
pub struct SimpleState(Option<AssetUuid>);
#[derive(TypeUuid)]
#[uuid = "720d636b-b79c-42d4-8f46-a2d8e1ada46e"]
pub struct ImageImporter;
impl AsyncImporter for ImageImporter {
    fn version_static() -> u32
    where
        Self: Sized,
    {
        1
    }
    fn version(&self) -> u32 {
        Self::version_static()
    }

    type Options = ();

    type State = SimpleState;

    /// Reads the given bytes and produces assets.
    fn import<'a>(
        &'a self,
        source: &'a mut (dyn AsyncRead + Unpin + Send + Sync),
        _options: Self::Options,
        state: &'a mut Self::State,
    ) -> BoxFuture<'a, Result<ImporterValue>> {
        Box::pin(async move {
            let id = state
                .0
                .unwrap_or_else(|| AssetUuid(*uuid::Uuid::new_v4().as_bytes()));
            *state = SimpleState(Some(id));
            let mut bytes = Vec::new();
            source.read_to_end(&mut bytes).await?;
            let asset =
                Image::Rgb8(image2::io::decode(&bytes).map_err(|e| Error::Boxed(Box::new(e)))?);
            Ok(ImporterValue {
                assets: vec![ImportedAsset {
                    id,
                    search_tags: vec![],
                    build_deps: vec![],
                    load_deps: vec![],
                    build_pipeline: None,
                    asset_data: Box::new(asset),
                }],
            })
        })
    }
}

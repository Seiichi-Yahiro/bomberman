use crate::assets::{Arena, Arenas};
use amethyst::{
    assets::{Asset, Format, Handle, RonFormat},
    audio::{OggFormat, Source},
    ui::{FontAsset, TtfFormat},
};
use std::marker::PhantomData;

macro_rules! loadable_asset_types {
    ($($name:ident : $asset:ty),+) => {

        #[derive(Clone)]
        pub enum LoadableAssetType {
            $(
                $name(Handle<$asset>)
            ),+
        }

        $(
            impl From<Handle<$asset>> for LoadableAssetType {
                fn from(handle: Handle<$asset>) -> Self {
                    LoadableAssetType::$name(handle)
                }
            }

            impl Into<Handle<$asset>> for LoadableAssetType {
                fn into(self) -> Handle<$asset> {
                    if let LoadableAssetType::$name(handle) = self {
                        handle
                    } else {
                        unreachable!();
                    }
                }
            }
        )+
    };
}

loadable_asset_types!(Font: FontAsset, Sound: Source, Arenas: Arenas, Arena: Arena);

pub struct AssetData<'f, A, F>
where
    A: Asset,
    F: Format<A::Data>,
{
    pub name: &'static str,
    pub filename: &'f str,
    pub format: F,
    phantom: PhantomData<A>,
}

impl<A, F> AssetData<'_, A, F>
where
    A: Asset,
    F: Format<A::Data>,
{
    pub fn new<'f>(name: &'static str, filename: &'f str, format: F) -> AssetData<'f, A, F> {
        AssetData {
            name,
            filename,
            format,
            phantom: PhantomData,
        }
    }
}

pub struct AssetDatas {
    pub font_main: AssetData<'static, FontAsset, TtfFormat>,
    pub sfx_cursor: AssetData<'static, Source, OggFormat>,
    pub custom_arenas: AssetData<'static, Arenas, RonFormat>,
}

pub static ASSET_DATAS: AssetDatas = AssetDatas {
    font_main: AssetData {
        name: "font_main",
        filename: "fonts/verdana.ttf",
        format: TtfFormat,
        phantom: PhantomData,
    },
    sfx_cursor: AssetData {
        name: "sfx_cursor",
        filename: "sfx/cursor.ogg",
        format: OggFormat,
        phantom: PhantomData,
    },
    custom_arenas: AssetData {
        name: "custom_arenas",
        filename: "arenas/arenas.ron",
        format: RonFormat,
        phantom: PhantomData,
    },
};

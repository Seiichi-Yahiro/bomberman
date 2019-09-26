use crate::assets::Arenas;
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

loadable_asset_types!(Font: FontAsset, Sound: Source, Arenas: Arenas);

pub struct AssetData<A, F>
where
    A: Asset,
    F: Format<A::Data>,
{
    pub name: &'static str,
    pub filename: &'static str,
    pub format: F,
    phantom: PhantomData<A>,
}

pub struct AssetDatas {
    pub font_main: AssetData<FontAsset, TtfFormat>,
    pub sfx_cursor: AssetData<Source, OggFormat>,
    pub custom_arenas: AssetData<Arenas, RonFormat>,
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

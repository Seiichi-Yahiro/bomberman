use crate::assets::Arenas;
use amethyst::{assets::Handle, audio::Source, ui::FontAsset};

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

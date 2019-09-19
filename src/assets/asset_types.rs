use crate::assets::Arenas;
use amethyst::{assets::Handle, audio::Source, ui::FontAsset};

#[derive(Clone)]
pub enum AssetType {
    Font(Handle<FontAsset>),
    Sound(Handle<Source>),
    Arenas(Handle<Arenas>),
}

impl From<Handle<FontAsset>> for AssetType {
    fn from(handle: Handle<FontAsset>) -> Self {
        AssetType::Font(handle)
    }
}

impl From<Handle<Source>> for AssetType {
    fn from(handle: Handle<Source>) -> Self {
        AssetType::Sound(handle)
    }
}

impl From<Handle<Arenas>> for AssetType {
    fn from(handle: Handle<Arenas>) -> Self {
        AssetType::Arenas(handle)
    }
}

impl Into<Handle<FontAsset>> for AssetType {
    fn into(self) -> Handle<FontAsset> {
        if let AssetType::Font(handle) = self {
            handle
        } else {
            panic!()
        }
    }
}

impl Into<Handle<Source>> for AssetType {
    fn into(self) -> Handle<Source> {
        if let AssetType::Sound(handle) = self {
            handle
        } else {
            panic!()
        }
    }
}

impl Into<Handle<Arenas>> for AssetType {
    fn into(self) -> Handle<Arenas> {
        if let AssetType::Arenas(handle) = self {
            handle
        } else {
            panic!()
        }
    }
}

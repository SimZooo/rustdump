use std::borrow::Cow;

use anyhow::anyhow;
use gpui::{AssetSource, SharedString};
use gpui_component_assets::Assets;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
#[include = "images/**/*.png"]
#[include = "fonts/**/*.ttf"]
pub struct CustomAssets;

impl AssetSource for CustomAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow::anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

pub struct CombinedAssets {
    main_assets: Assets,
    custom_assets: CustomAssets,
}

impl CombinedAssets {
    pub fn new() -> Self {
        Self {
            main_assets: Assets,
            custom_assets: CustomAssets,
        }
    }
}

impl AssetSource for CombinedAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        // Try main assets first
        match self.main_assets.load(path) {
            Ok(Some(data)) => return Ok(Some(data)),
            Ok(None) => {}
            Err(_) => {}
        }

        // Try sidebar assets
        match self.custom_assets.load(path) {
            Ok(Some(data)) => Ok(Some(data)),
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow!("Failed to load asset from both sources: {}", e)),
        }
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut all_assets = self.main_assets.list(path)?;
        let sidebar_assets = self.custom_assets.list(path)?;
        all_assets.extend(sidebar_assets);
        Ok(all_assets)
    }
}

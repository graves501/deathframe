use std::collections::HashMap;
use std::path::Path;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::World;
use amethyst::renderer::{
    SpriteSheet,
    SpriteSheetFormat,
    SpriteSheetHandle,
    Texture,
    TextureFormat,
    TextureMetadata,
};
use regex::{Regex, RegexBuilder};

/// This is a resource wrapper for amethyst's `SpriteSheet`s.
/// It can load and get `SpriteSheet`s;
/// _load_ them by passing a spritesheet's image file path to an appropriate method and
/// _get_ them by passing their spritesheet's image file name (without extension) to an appropriate
/// method.
pub struct SpriteSheetHandles {
    spritesheet_handles: HashMap<String, SpriteSheetHandle>,
}

impl SpriteSheetHandles {
    /// Insert a new `SpriteSheetHandle` with an identifier name into this resource.
    /// You will not usually use this method, instead use a method such as `load`,
    /// which handles this for you.
    pub fn insert<T: ToString>(&mut self, name: T, handle: SpriteSheetHandle) {
        self.spritesheet_handles.insert(name.to_string(), handle);
    }

    /// Get the `SpriteSheetHandle` with the given identifier name.
    /// Returns `None` if there is no `SpriteSheetHandle` with this name,
    /// and returns `Some` with the `SpriteSheetHandle` if there is.
    pub fn get<T: ToString>(&self, name: T) -> Option<SpriteSheetHandle> {
        let name = name.to_string();
        let err_msg =
            format!("Given SpriteSheetHandle name cannot be empty: {}", name);
        // Get the basename of the file (without the extension), in case a path is passed
        let name = name
            .split("/")
            .last()
            .expect(&err_msg)
            .split(".")
            .next()
            .expect(&err_msg);
        self.spritesheet_handles.get(name).map(Clone::clone)
    }

    /// Load a new `SpriteSheet` and `SpriteSheetHandle` into this resource
    /// by passing the path to the spritesheet image file to this method (and the world).
    pub fn load<P>(&mut self, path: P, world: &World)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        if !path.is_file() {
            panic!(format!(
                "Given image file path does not exist: '{:?}'",
                path
            ));
        }
        let err_msg_match = format!(
            "Given image file path must match the pattern \
             `/.+\\.(png|jpe?g)\\z/i`. Given: '{:?}'",
            path
        );
        let filepath_regex = RegexBuilder::new(r".+\.(png|jpe?g)\z")
            .case_insensitive(true)
            .build()
            .unwrap();

        if let Some(capture) = filepath_regex.captures(path.to_str().unwrap()) {
            let extension =
                path.extension().expect(&err_msg_match).to_str().unwrap();
            let extension_with_dot = format!(".{}", extension);
            let name = path
                .file_name()
                .expect(&format!(
                    "Given path must lead to an image file. Given: '{:?}'",
                    path
                ))
                .to_str()
                .unwrap()
                .replace(&extension_with_dot, "");
            let path_ron_string =
                path.to_str().unwrap().replace(&extension_with_dot, ".ron");
            let path_ron = Path::new(path_ron_string.as_str());
            if !path_ron.is_file() {
                panic!(format!(
                    "Given image file path does not have a .ron configuration \
                     file: '{:?}'",
                    path_ron
                ));
            }

            let image_format = match extension.to_lowercase().as_str() {
                "png" => TextureFormat::Png,
                "jpg" | "jpeg" => TextureFormat::Jpg,
                "bmp" => TextureFormat::Bmp,
                "tga" => TextureFormat::Tga,
                ext => panic!(format!(
                    "Given format is not supported for images: '{:?}'",
                    ext
                )),
            };

            let handle = {
                let loader = world.read_resource::<Loader>();
                let texture_handle = {
                    let texture_storage =
                        world.read_resource::<AssetStorage<Texture>>();
                    loader.load(
                        path.to_str().unwrap(),
                        image_format,
                        TextureMetadata::srgb_scale(),
                        (),
                        &texture_storage,
                    )
                };
                let spritesheet_store =
                    world.read_resource::<AssetStorage<SpriteSheet>>();
                loader.load(
                    path_ron.to_str().unwrap(),
                    SpriteSheetFormat,
                    texture_handle,
                    (),
                    &spritesheet_store,
                )
            };

            self.insert(name, handle);
        } else {
            panic!(err_msg_match)
        }
    }

    /// Get a `SpriteSheetHandle` with the given path to the spritesheet's image file.
    /// If it does not already exist, load it first, then return the newly loaded handle.
    pub fn get_or_load<T>(
        &mut self,
        path: T,
        world: &World,
    ) -> SpriteSheetHandle
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref().to_str().unwrap();
        if let Some(handle) = self.get(path) {
            handle
        } else {
            self.load(path, world);
            self.get(path).unwrap()
        }
    }

    /// Returns `true` if all `SpriteSheetHandle`s' textures have finished loading.
    pub fn has_finished_loading_all(&self, world: &World) -> bool {
        let asset = world.read_resource::<AssetStorage<SpriteSheet>>();
        self.spritesheet_handles
            .values()
            .all(|handle| asset.get(handle).is_some())
    }
}

impl Default for SpriteSheetHandles {
    fn default() -> Self {
        Self {
            spritesheet_handles: HashMap::new(),
        }
    }
}

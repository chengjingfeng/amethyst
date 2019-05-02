use serde::{Deserialize, Serialize};
use amethyst_assets::{PrefabData, Format, Asset};

pub trait UiRenderer: Send + Sync + 'static + Clone {
    type Texture: Send + Sync + Asset;
}

pub trait TexturePrefab<R, F>: Send + Sync + 'static +
    Clone + Serialize + for<'de> Deserialize<'de> + for<'a> PrefabData<'a>
where
    R: UiRenderer,
    F: Format<R::Texture, Options = ()>,
{}
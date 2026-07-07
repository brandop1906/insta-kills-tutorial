
use bevy::prelude::*;
use serde::Deserialize;
use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
struct KeyframeRange { start: usize, end: usize }

#[derive(Debug, Clone, Deserialize)]
enum Keyframes {
    KeyframesRange(KeyframeRange),
    KeyframesVec(Vec<usize>),
}

impl Keyframes {
    fn into_indices(self) -> Vec<usize> {
        match self {
            Keyframes::KeyframesRange(r) => (r.start..r.end).collect(),
            Keyframes::KeyframesVec(v) => v,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct AnimationClipDef { name: String, keyframes: Keyframes, duration: f32 }

// The runtime clip: a flat list of frame indices + how long to play them.
#[derive(Asset, TypePath, Debug, Clone)]
pub struct AnimationClip2D {
    keyframes: Vec<usize>,
    duration: f32,
}

impl AnimationClip2D {
    fn frame_at(&self, seek_time: f32) -> usize {
        let len = self.keyframes.len();
        let idx = ((seek_time / self.duration) * len as f32) as usize;
        self.keyframes[idx.min(len - 1)]
    }
}

#[derive(Debug, Error)]
pub enum AnimationClip2DLoaderError {
    #[error("could not read animation manifest: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse animation manifest: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

#[derive(Default, TypePath)]
struct AnimationClip2DLoader;

impl AssetLoader for AnimationClip2DLoader {
    type Asset = AnimationClip2D;
    type Settings = ();
    type Error = AnimationClip2DLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let defs: Vec<AnimationClipDef> = ron::de::from_bytes(&bytes)?;

        let mut first = None;
        for def in defs {
            let clip = AnimationClip2D {
                keyframes: def.keyframes.into_indices(),
                duration: def.duration,
            };
            // Register each clip under its name, e.g. "...#idle".
            load_context.add_labeled_asset(def.name, clip.clone());
            if first.is_none() { first = Some(clip); }
        }
        Ok(first.expect("manifest must contain at least one clip"))
    }

    fn extensions(&self) -> &[&str] { &["trickfilm"] }
}

#[derive(Component)]
pub struct AnimationPlayer2D {
    clip: Handle<AnimationClip2D>,
    seek_time: f32,
    speed: f32,
    repeat: bool,
    finished: bool,
    // ...a couple more flags in the real file (reverse, just_finished_cycle)
}

// We write Default by hand instead of deriving it: a *derived* Default sets
// speed = 0.0, and animate_sprites advances by `delta * speed` — so the clip
// would sit frozen on frame 0 forever. We want forward playback out of the box.
impl Default for AnimationPlayer2D {
    fn default() -> Self {
        Self {
            clip: Handle::default(),
            seek_time: 0.0,
            speed: 1.0,
            repeat: false,
            finished: false,
        }
    }
}

impl AnimationPlayer2D {
    pub fn play(&mut self, clip: Handle<AnimationClip2D>) -> &mut Self {
        // Guard: if we're already playing this clip, don't restart it.
        // State systems call play() every frame with the current state's clip;
        // without this the animation would reset to frame 0 forever.
        if self.clip == clip { return self; }
        self.clip = clip;
        self.seek_time = 0.0;
        self.finished = false;
        self
    }
    pub fn repeat(&mut self) -> &mut Self { self.repeat = true; self }

    // Read-only accessor: chapter 7's `leave_dash` uses this to know when a
    // one-shot clip (like the dash) has played out.
    pub fn is_finished(&self) -> bool { self.finished }
}

fn animate_sprites(
    time: Res<Time>,
    clips: Res<Assets<AnimationClip2D>>,
    mut q: Query<(&mut AnimationPlayer2D, &mut Sprite)>,
) {
    for (mut player, mut sprite) in &mut q {
        if player.finished && !player.repeat { continue; }
        let Some(clip) = clips.get(&player.clip) else { continue; };
        let Some(atlas) = sprite.texture_atlas.as_mut() else { continue; };

        player.seek_time += time.delta_secs() * player.speed;
        if player.seek_time >= clip.duration {
            if player.repeat { player.seek_time -= clip.duration; }
            else { player.finished = true; player.seek_time = clip.duration; }
        }
        atlas.index = clip.frame_at(player.seek_time);  // the actual animation!
    }
}

pub struct Animation2DPlugin;

impl Plugin for Animation2DPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationClip2D>()
            .init_asset_loader::<AnimationClip2DLoader>()
            .add_systems(Update, animate_sprites);
    }
}
use bevy::prelude::*;

#[derive(Component)]
struct AsepriteMarker;

#[derive(Component)]
pub struct AsepriteIndices {
    first: usize,
    last: usize,
}
impl AsepriteIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AsepriteTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AsepritePlaying(pub bool);

#[derive(Bundle)]
pub struct Aseprite {
    pub sprite: Sprite,
    pub indices: AsepriteIndices,
    pub playing: AsepritePlaying,
    pub timer: AsepriteTimer,
    marker: AsepriteMarker,
}
impl Aseprite {
    pub fn new(
        sprite: Sprite,
        indices: AsepriteIndices,
        playing: AsepritePlaying,
        timer: AsepriteTimer,
    ) -> Self {
        Self {
            sprite,
            indices,
            playing,
            timer,
            marker: AsepriteMarker,
        }
    }

    pub fn with_filp_x(mut self, filp_x: bool) -> Self {
        self.sprite.flip_x = filp_x;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.sprite.custom_size = Some(size);
        self
    }
}
impl Default for Aseprite {
    fn default() -> Self {
        Self {
            sprite: Sprite::default(),
            indices: AsepriteIndices::new(0, 0),
            playing: AsepritePlaying(false),
            timer: AsepriteTimer(Timer::default()),
            marker: AsepriteMarker,
        }
    }
}

type AsepriteQuery<'a, 'b, 'c> = Query<
    'a,
    'b,
    (
        &'c mut Sprite,
        &'c AsepriteIndices,
        &'c AsepritePlaying,
        &'c mut AsepriteTimer,
    ),
    With<AsepriteMarker>,
>;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsepriteSystemState {
    #[default]
    Paused,
    Running,
}

pub struct AsepritePlugin;
impl AsepritePlugin {
    fn update(time: Res<Time>, aseprite_query: AsepriteQuery) {
        for (mut sprite, indices, playing, mut timer) in aseprite_query {
            if !**playing {
                continue;
            }
            timer.tick(time.delta());
            if timer.just_finished() {
                if let Some(ref mut atlas) = sprite.texture_atlas {
                    atlas.index = if atlas.index == indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}
impl Plugin for AsepritePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AsepriteSystemState>().add_systems(
            Update,
            Self::update.run_if(in_state(AsepriteSystemState::Running)),
        );
    }
}

use super::*;
use bevy::{asset::RenderAssetUsages, prelude::*};

#[derive(Component)]
#[require(
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
)]
struct MapGrid;

#[derive(Event)]
pub struct GridCreateEvent {
    pub rows: usize,
    pub cols: usize,
    pub color: [f32; 4],
}

pub struct TilesPlugin;
impl TilesPlugin {
    const SPACING: f32 = 32.0;

    fn create_grid(
        trigger: Trigger<GridCreateEvent>,
        mut command: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        grid_entities: Query<Entity, With<MapGrid>>,
    ) {
        for entity in grid_entities {
            command.entity(entity).despawn();
        }

        let mut mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        );

        let mut positions = Vec::new();
        let mut colors = Vec::new();
        for col in 0..=trigger.cols {
            let x = col as f32 * Self::SPACING;
            positions.push([x, 0.0, 0.0]);
            positions.push([x, (trigger.rows as f32) * Self::SPACING, 0.0]);
            colors.push(trigger.color);
            colors.push(trigger.color);
        }

        for row in 0..=trigger.rows {
            let y = row as f32 * Self::SPACING;
            positions.push([0.0, y, 0.0]);
            positions.push([(trigger.cols as f32) * Self::SPACING, y, 0.0]);
            colors.push(trigger.color);
            colors.push(trigger.color);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

        command.spawn((
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
            MapGrid,
        ));
    }

    fn parse(
        mut command: Commands,
        window: Single<&Window>,
        level_resource: Res<LevelResource>,
        asset: Res<Assets<LevelAsset>>,
        mut next_state: ResMut<NextState<AsepriteSystemState>>,
    ) {
        let level_asset = asset.get(&level_resource.data_handle).unwrap();

        command.trigger(GridCreateEvent {
            rows: level_asset.rows,
            cols: level_asset.cols,
            color: [1.0, 0.0, 0.0, 1.0],
        });

        command.spawn((
            Camera2d,
            Transform::from_translation(Vec3::new(
                window.resolution.width() / 2.0,
                window.resolution.height() / 2.0,
                3.0,
            )),
        ));

        for descriptor in &level_asset.data {
            let translation = Vec3::new(descriptor.tile_pos.0, descriptor.tile_pos.1, 0.0);
            match descriptor.tile_typ {
                TileType::Pass => {
                    command.spawn(PassBox::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ));
                }
                TileType::Wall => {
                    command.spawn(Floor::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ));
                }
                TileType::Trap => {
                    command.spawn(HitBox::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ));
                }
            }
        }
        next_state.set(AsepriteSystemState::Running);
    }
}
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridCreateEvent>()
            .add_observer(Self::create_grid)
            .add_systems(OnEnter(UIState::Running), Self::parse);
    }
}

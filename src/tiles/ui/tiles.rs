use super::*;
use bevy::{
    asset::RenderAssetUsages, input::mouse::MouseWheel, render::camera::Viewport,
    window::WindowResized,
};

#[derive(Component)]
struct TilesMarker;

#[derive(Component)]
#[require(
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
)]
struct MapGrid;

#[derive(Event)]
struct GridCreateEvent {
    pub rows: usize,
    pub cols: usize,
    pub color: [f32; 4],
}

#[derive(Component)]
#[require(Sprite)]
struct CurrentTile;

pub(super) struct TilesPlugin;
impl TilesPlugin {
    const TILE_SIZE: f32 = 32.0;
    const SPACING: f32 = Self::TILE_SIZE;
    const COLOR_VERTEX: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const TILEMAP_SCALE_RANGE: (f32, f32) = (0.65, 1.5);

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
        window: Single<&Window>,
        mut command: Commands,
        mut map_data: ResMut<MapData>,
        level_resource: Res<LevelResource>,
        asset: Res<Assets<LevelAsset>>,
        mut next_state: ResMut<NextState<AsepriteSystemState>>,
    ) {
        let level_asset = asset.get(&level_resource.data_handle).unwrap();
        command.trigger(GridCreateEvent {
            rows: level_asset.rows,
            cols: level_asset.cols,
            color: Self::COLOR_VERTEX,
        });
        map_data.id = level_resource.id;
        map_data.rows = level_asset.rows;
        map_data.cols = level_asset.cols;
        map_data.entry = level_asset.entry;
        map_data.next = level_asset.next;

        for descriptor in &level_asset.data {
            let translation = Vec3::new(descriptor.tile_pos.0, descriptor.tile_pos.1, 0.0);
            let id = match descriptor.tile_typ {
                TileType::Pass => command
                    .spawn(PassBox::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ))
                    .id(),
                TileType::Wall => command
                    .spawn(Floor::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ))
                    .id(),
                TileType::Trap => command
                    .spawn(HitBox::new(
                        translation,
                        descriptor.rotation,
                        &level_resource,
                    ))
                    .id(),
            };
            map_data.data.insert(
                translation.truncate().as_uvec2(),
                TileData {
                    id,
                    typ: descriptor.tile_typ,
                    rotation: descriptor.rotation,
                },
            );
        }
        command.spawn((
            Camera2d,
            Camera {
                order: UIPlugin::TILE_VIEWPORT_ORDER,
                viewport: Some(Viewport {
                    physical_position: UIPlugin::TILE_VIEWPORT_START.as_uvec2(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            Transform::from_translation(
                (window.size() * UIPlugin::TILE_VIEWPORT_VAL * 0.5).extend(3.0),
            ),
            TilesMarker,
        ));
        next_state.set(AsepriteSystemState::Running);
    }

    //fix a bevy's bug
    fn resize(
        window: Single<&Window>,
        mut window_event: EventReader<WindowResized>,
        mut camera: Single<&mut Camera, With<TilesMarker>>,
    ) {
        for _ in window_event.read() {
            if let Some(ref mut viewport) = camera.viewport {
                viewport.physical_size =
                    (window.size() * window.scale_factor() * UIPlugin::TILE_VIEWPORT_VAL)
                        .as_uvec2();
            }
        }
    }

    fn tracking(
        window: Single<&Window>,
        mut camera_transform: Single<&mut Transform, With<TilesMarker>>,
        mouse_button: Res<ButtonInput<MouseButton>>,
        mut mouse_drag: EventReader<CursorMoved>,
    ) {
        if !mouse_button.pressed(MouseButton::Left) || mouse_button.get_pressed().len() != 1 {
            return;
        }
        for drag in mouse_drag.read() {
            if !Rect::from_corners(
                UIPlugin::TILE_VIEWPORT_START,
                window.size() * UIPlugin::TILE_VIEWPORT_VAL,
            )
            .contains(drag.position)
            {
                continue;
            }
            if let Some(mut delta) = drag.delta {
                delta.x = -delta.x;
                camera_transform.translation += delta.extend(0.0);
            }
        }
    }

    fn scale(
        key: Res<ButtonInput<KeyCode>>,
        mut mouse_wheel: EventReader<MouseWheel>,
        mut camera_projection: Single<&mut Projection, With<TilesMarker>>,
    ) {
        if !key.pressed(KeyCode::ControlLeft) {
            return;
        }
        for wheel_event in mouse_wheel.read() {
            if let Projection::Orthographic(projection) = &mut **camera_projection {
                if wheel_event.y > 0.0 && projection.scale > Self::TILEMAP_SCALE_RANGE.0 {
                    projection.scale -= 0.05;
                } else if wheel_event.y < 0.0 && projection.scale < Self::TILEMAP_SCALE_RANGE.1 {
                    projection.scale += 0.05;
                }
            }
        }
    }

    fn align_and_offset(translation: Vec3) -> Vec3 {
        ((translation.as_uvec3() / 32) * 32).as_vec3() + 16.0
    }

    fn get_real_translation(
        window: Single<&Window>,
        camera_transform: Single<&Transform, With<TilesMarker>>,
        camera_projection: Single<&Projection, With<TilesMarker>>,
    ) -> Option<Vec3> {
        let viewport_area = Rect::from_corners(
            UIPlugin::TILE_VIEWPORT_START,
            window.size() * UIPlugin::TILE_VIEWPORT_VAL,
        );
        let cursor_position = window.cursor_position()?;
        if !viewport_area.contains(cursor_position) {
            return None;
        }
        let scale = if let Projection::Orthographic(projection) = &**camera_projection {
            projection.scale
        } else {
            return None;
        };
        Some(Self::align_and_offset(
            (camera_transform.translation.truncate()
                + ((cursor_position - viewport_area.center()) * Vec2::new(scale, -scale)))
            .extend(0.0),
        ))
    }

    fn selected(
        mut command: Commands,
        window: Single<&Window>,
        selected: Res<Selected>,
        mut map_data: ResMut<MapData>,
        level_resource: Res<LevelResource>,
        mouse_buttons: Res<ButtonInput<MouseButton>>,
        camera_transform: Single<&Transform, With<TilesMarker>>,
        camera_projection: Single<&Projection, With<TilesMarker>>,
    ) {
        if !mouse_buttons.pressed(MouseButton::Left) || mouse_buttons.get_pressed().len() != 1 {
            return;
        }
        let real_translation = if let Some(real_translation) =
            Self::get_real_translation(window, camera_transform, camera_projection)
        {
            real_translation
        } else {
            return;
        };
        let key = real_translation.truncate().as_uvec2();

        if let Some(tile_data) = map_data.data.get(&key) {
            if selected.typ == tile_data.typ {
                return;
            } else {
                command.entity(tile_data.id).despawn();
                map_data.data.remove(&key);
            }
        }

        let id = match selected.typ {
            TileType::Wall => command
                .spawn(Floor::new(real_translation, 0.0, &level_resource))
                .id(),
            TileType::Pass => command
                .spawn(PassBox::new(real_translation, 0.0, &level_resource))
                .id(),
            TileType::Trap => command
                .spawn(HitBox::new(real_translation, 0.0, &level_resource))
                .id(),
        };
        map_data.data.insert(
            real_translation.truncate().as_uvec2(),
            TileData {
                id,
                typ: selected.typ,
                rotation: 0.0,
            },
        );
    }

    fn earse(
        mut command: Commands,
        window: Single<&Window>,
        mut map_data: ResMut<MapData>,
        mouse_buttons: Res<ButtonInput<MouseButton>>,
        camera_transform: Single<&Transform, With<TilesMarker>>,
        camera_projection: Single<&Projection, With<TilesMarker>>,
    ) {
        if !mouse_buttons.pressed(MouseButton::Right) || mouse_buttons.get_pressed().len() != 1 {
            return;
        }
        if let Some(real_translation) =
            Self::get_real_translation(window, camera_transform, camera_projection)
        {
            if let Some(tile_data) = map_data
                .data
                .remove(&real_translation.truncate().as_uvec2())
            {
                command.entity(tile_data.id).despawn();
            }
        }
    }
}
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridCreateEvent>()
            .add_observer(Self::create_grid)
            .add_systems(OnEnter(UIState::Running), Self::parse)
            .add_systems(
                Update,
                (Self::resize, Self::scale, Self::earse).run_if(in_state(UIState::Running)),
            )
            .add_systems(
                Update,
                Self::tracking.run_if(in_state(EditorState::Tracking)),
            )
            .add_systems(
                Update,
                Self::selected.run_if(in_state(EditorState::Selected)),
            );
    }
}

use super::super::tile::prelude::*;
use super::*;

pub struct PlayerManager;
impl PlayerManager {
    const VELOCITY_SPEED: f32 = 120.0;
    const JUMP_SPEED: f32 = 233.0;

    fn init(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut command: Commands,
        level_resource: Res<LevelResource>,
        level_config: Res<Assets<LevelAsset>>,
        mut next_state: ResMut<NextState<PlayerState>>,
    ) {
        let shadow = Shadow::new(
            meshes.add(Capsule2d::new(10.0, 14.0)),
            materials.add(Color::srgba_u8(0, 0, 0, 120)),
        );
        let player = Player::new(
            level_config
                .get(&level_resource.data_handle)
                .unwrap()
                .entry
                .extend(2.0),
        );
        command
            .spawn((player, StateScoped(PlayerState::Running)))
            .with_child(shadow)
            .with_children(PlayerCheckers::add_to)
            .observe(Self::hurt);
        next_state.set(PlayerState::Running);
    }

    fn hurt(
        trigger: Trigger<OnCollisionStart>,
        hitbox: Query<&HitBoxMarker>,
        mut hp: PlayerHPQuery,
        mut level_event: EventWriter<LevelPass>,
        mut next_scene: ResMut<NextState<GameScene>>,
    ) {
        if hitbox.contains(trigger.collider) {
            if hp.0 == 1 {
                level_event.write(LevelPass(false));
                next_scene.set(GameScene::GameOver);
                return;
            }
            hp.0 -= 1;
        }
    }

    fn handle_input(
        state: Res<State<PlayerRunningState>>,
        input: Res<ButtonInput<KeyCode>>,
        front_wall_query: FrontWallQuery,
        back_wall_query: BackWallQuery,
        mut sprite: Single<&mut Sprite, With<PlayerMarker>>,
        mut next_state: ResMut<NextState<PlayerRunningState>>,
        mut player_linear_velocity_query: PlayerLinearVelocityQuery,
    ) {
        player_linear_velocity_query.x = 0.0;
        if [PlayerRunningState::Walk, PlayerRunningState::Idle].contains(state.get())
            && input.just_pressed(KeyCode::Space)
        {
            player_linear_velocity_query.y = Self::JUMP_SPEED;
            next_state.set(PlayerRunningState::Jump);
        }
        if input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft])
            && back_wall_query.iter().all(|hits| hits.is_empty())
        {
            sprite.flip_x = false;
            player_linear_velocity_query.x = -Self::VELOCITY_SPEED;
            return;
        }
        if input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight])
            && front_wall_query.iter().all(|hits| hits.is_empty())
        {
            sprite.flip_x = true;
            player_linear_velocity_query.x = Self::VELOCITY_SPEED;
        }
    }

    fn enter_fall(
        mut player_aseprite_param: PlayerAsepriteQuery,
        player_resource: Res<PlayerResource>,
    ) {
        let (image_handle, layout_handle) = player_resource
            .texture_atlas_handles
            .get(&PlayerAsepriteType::Fall)
            .unwrap();
        player_aseprite_param.sprite.image = image_handle.clone();
        player_aseprite_param.sprite.texture_atlas = Some(TextureAtlas {
            layout: layout_handle.clone(),
            index: 0,
        });
        *player_aseprite_param.playing = AsepritePlaying(false);
    }

    fn on_fall(
        time: Res<Time>,
        gravity: Res<Gravity>,
        ground_query: GroundQuery,
        mut next_state: ResMut<NextState<PlayerRunningState>>,
        mut player_linear_velocity_query: PlayerLinearVelocityQuery,
    ) {
        if ground_query.iter().any(|hits| !hits.is_empty()) {
            next_state.set(PlayerRunningState::Idle);
            return;
        }
        player_linear_velocity_query.0 += gravity.0 * time.delta_secs();
    }

    fn enter_idle(
        mut player_aseprite_param: PlayerAsepriteQuery,
        player_resource: Res<PlayerResource>,
    ) {
        let (image_handle, layout_handle) = player_resource
            .texture_atlas_handles
            .get(&PlayerAsepriteType::Idle)
            .unwrap();
        player_aseprite_param.sprite.image = image_handle.clone();
        player_aseprite_param.sprite.texture_atlas = Some(TextureAtlas {
            layout: layout_handle.clone(),
            index: 0,
        });
        *player_aseprite_param.playing = AsepritePlaying(false);
    }

    fn on_idle(
        player_linear_velocity_query: PlayerLinearVelocityQuery,
        mut next_state: ResMut<NextState<PlayerRunningState>>,
    ) {
        if player_linear_velocity_query.x.abs() == Self::VELOCITY_SPEED {
            next_state.set(PlayerRunningState::Walk);
        }
    }

    fn enter_jump(
        mut player_aseprite_param: PlayerAsepriteQuery,
        player_resource: Res<PlayerResource>,
    ) {
        let (image_handle, layout_handle) = player_resource
            .texture_atlas_handles
            .get(&PlayerAsepriteType::Jump)
            .unwrap();
        player_aseprite_param.sprite.image = image_handle.clone();
        player_aseprite_param.sprite.texture_atlas = Some(TextureAtlas {
            layout: layout_handle.clone(),
            index: 0,
        });
        *player_aseprite_param.indices =
            AsepriteIndices::new(0, PlayerAsepriteType::Jump.frame_count() - 1);
        *player_aseprite_param.timer = AsepriteTimer(Timer::from_seconds(0.10, TimerMode::Once));
        *player_aseprite_param.playing = AsepritePlaying(true);
    }

    fn on_jump(
        input: Res<ButtonInput<KeyCode>>,
        mut player_linear_velocity_query: PlayerLinearVelocityQuery,
        mut next_running_state: ResMut<NextState<PlayerRunningState>>,
    ) {
        if input.just_released(KeyCode::Space) && player_linear_velocity_query.0.y > 0.0 {
            player_linear_velocity_query.y = 0.0;
        }
        if player_linear_velocity_query.y <= 0.0 {
            next_running_state.set(PlayerRunningState::Fall);
            return;
        }
    }

    fn enter_walk(
        mut player_aseprite_param: PlayerAsepriteQuery,
        player_resource: Res<PlayerResource>,
    ) {
        let (image_handle, layout_handle) = player_resource
            .texture_atlas_handles
            .get(&PlayerAsepriteType::Walk)
            .unwrap();
        player_aseprite_param.sprite.image = image_handle.clone();
        player_aseprite_param.sprite.texture_atlas = Some(TextureAtlas {
            layout: layout_handle.clone(),
            index: 0,
        });
        *player_aseprite_param.timer =
            AsepriteTimer(Timer::from_seconds(0.1, TimerMode::Repeating));
        *player_aseprite_param.indices =
            AsepriteIndices::new(0, PlayerAsepriteType::Walk.frame_count() - 1);
        *player_aseprite_param.playing = AsepritePlaying(true);
    }

    fn on_walk(
        ground_query: GroundQuery,
        player_linear_velocity_query: PlayerLinearVelocityQuery,
        mut next_state: ResMut<NextState<PlayerRunningState>>,
    ) {
        if ground_query.iter().all(|hits| hits.is_empty()) {
            next_state.set(PlayerRunningState::Fall);
            return;
        }
        if player_linear_velocity_query.x == 0.0 {
            next_state.set(PlayerRunningState::Idle);
            return;
        }
    }

    fn render_rays(rays: Query<(&RayCaster, &RayHits)>, mut gizmos: Gizmos) {
        for (ray, hits) in rays {
            let origin = ray.global_origin();
            let direction = ray.global_direction().as_vec2();
            gizmos.line_2d(
                origin,
                origin + direction * ray.max_distance,
                if hits.is_empty() {
                    Color::srgb_u8(200, 0, 0)
                } else {
                    Color::srgb_u8(0, 200, 0)
                },
            );
        }
    }
}
impl Plugin for PlayerManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::new(0.0, -300.0)))
            .add_sub_state::<PlayerState>()
            .add_systems(OnEnter(PlayerState::Loading), Self::init)
            .add_sub_state::<PlayerRunningState>()
            .add_systems(
                Update,
                Self::render_rays.run_if(in_state(PlayerState::Running)),
            )
            .add_systems(OnEnter(PlayerRunningState::Fall), Self::enter_fall)
            .add_systems(
                Update,
                Self::handle_input.run_if(in_state(PlayerState::Running)),
            )
            .add_systems(
                Update,
                Self::on_fall.run_if(in_state(PlayerRunningState::Fall)),
            )
            .add_systems(OnEnter(PlayerRunningState::Idle), Self::enter_idle)
            .add_systems(
                Update,
                Self::on_idle.run_if(in_state(PlayerRunningState::Idle)),
            )
            .add_systems(OnEnter(PlayerRunningState::Walk), Self::enter_walk)
            .add_systems(
                Update,
                Self::on_walk.run_if(in_state(PlayerRunningState::Walk)),
            )
            .add_systems(OnEnter(PlayerRunningState::Jump), Self::enter_jump)
            .add_systems(
                Update,
                Self::on_jump.run_if(in_state(PlayerRunningState::Jump)),
            );
    }
}

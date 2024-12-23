use bevy::app::Plugin;
use bevy::asset::AssetId;
use bevy::color::Hsla;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::ecs::system::Resource;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::render::sync_world::RenderEntity;
use bevy::render::sync_world::TemporaryRenderEntity;
use bevy::render::Extract;
use bevy::render::ExtractSchedule;
use bevy::render::RenderApp;
use bevy::sprite::BorderRect;
use bevy::transform::components::GlobalTransform;

use bevy::ui::ComputedNode;
use bevy::ui::DefaultUiCamera;

use bevy::ui::ExtractedUiItem;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::NodeType;
use bevy::ui::TargetCamera;

/// Configuration for the UI debug overlay
#[derive(Resource)]
pub struct UiDebugOverlay {
    /// Set to true to enable the UI debug overlay
    pub enabled: bool,
    /// Width of the overlay's lines in logical pixels
    pub line_width: f32,
}

impl UiDebugOverlay {
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

impl Default for UiDebugOverlay {
    fn default() -> Self {
        Self {
            enabled: false,
            line_width: 1.,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn extract_debug_overlay(
    mut commands: Commands,
    debug_overlay: Extract<Res<UiDebugOverlay>>,
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    default_ui_camera: Extract<DefaultUiCamera>,
    uinode_query: Extract<
        Query<(
            Entity,
            &ComputedNode,
            &GlobalTransform,
            Option<&TargetCamera>,
        )>,
    >,
    mapping: Extract<Query<RenderEntity>>,
) {
    if !debug_overlay.enabled {
        return;
    }

    for (entity, uinode, transform, camera) in &uinode_query {
        let Some(camera_entity) = camera.map(TargetCamera::entity).or(default_ui_camera.get())
        else {
            continue;
        };

        let Ok(render_camera_entity) = mapping.get(camera_entity) else {
            continue;
        };

        // Extract a border box to display an outline for every UI Node in the layout
        extracted_uinodes.uinodes.insert(
            commands.spawn(TemporaryRenderEntity).id(),
            ExtractedUiNode {
                // Add a large number to the UI node's stack index so that the overlay is always drawn on top
                stack_index: uinode.stack_index() + u32::MAX / 2,
                color: Hsla::sequential_dispersed(entity.index()).into(),
                rect: Rect {
                    min: Vec2::ZERO,
                    max: uinode.size(),
                },
                clip: None,
                image: AssetId::default(),
                camera_entity: render_camera_entity,
                item: ExtractedUiItem::Node {
                    atlas_scaling: None,
                    transform: transform.compute_matrix(),
                    flip_x: false,
                    flip_y: false,
                    border: BorderRect::square(
                        debug_overlay.line_width / uinode.inverse_scale_factor(),
                    ),
                    border_radius: uinode.border_radius(),
                    node_type: NodeType::Border,
                },
                main_entity: entity.into(),
            },
        );
    }
}

pub struct UiDebugOverlayPlugin;

impl Plugin for UiDebugOverlayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<UiDebugOverlay>();

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(ExtractSchedule, extract_debug_overlay);
        }
    }
}

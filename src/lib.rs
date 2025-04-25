use bevy_app::Plugin;
use bevy_asset::AssetId;
use bevy_color::Hsla;
use bevy_ecs::entity::Entity;
#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectResource;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::Commands;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
use bevy_math::Rect;
use bevy_math::Vec2;
use bevy_render::sync_world::RenderEntity;
use bevy_render::sync_world::TemporaryRenderEntity;
use bevy_render::view::ViewVisibility;
use bevy_render::Extract;
use bevy_render::ExtractSchedule;
use bevy_render::RenderApp;
use bevy_sprite::BorderRect;
use bevy_transform::components::GlobalTransform;
use bevy_ui::CalculatedClip;
use bevy_ui::ComputedNode;
use bevy_ui::DefaultUiCamera;

use bevy_ui::ExtractedUiItem;
use bevy_ui::ExtractedUiNode;
use bevy_ui::ExtractedUiNodes;
use bevy_ui::NodeType;
use bevy_ui::UiTargetCamera;

/// Configuration for the UI debug overlay
#[derive(Resource)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Resource)
)]
pub struct UiDebugOverlay {
    /// Set to true to enable the UI debug overlay
    pub enabled: bool,
    /// Width of the overlay's lines in logical pixels
    pub line_width: f32,
    /// Show outlines for non-visible UI nodes
    pub show_hidden: bool,
    /// Show outlines for clipped sections of UI nodes
    pub show_clipped: bool,
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
            show_hidden: false,
            show_clipped: false,
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn extract_debug_overlay(
    mut commands: Commands,
    debug_overlay: Extract<Res<UiDebugOverlay>>,
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    default_ui_camera: Extract<DefaultUiCamera>,
    uinode_query: Extract<
        Query<(
            Entity,
            &ComputedNode,
            &ViewVisibility,
            Option<&CalculatedClip>,
            &GlobalTransform,
            Option<&UiTargetCamera>,
        )>,
    >,
    mapping: Extract<Query<RenderEntity>>,
) {
    if !debug_overlay.enabled {
        return;
    }

    for (entity, uinode, visibility, maybe_clip, transform, camera) in &uinode_query {
        if !debug_overlay.show_hidden && !visibility.get() {
            continue;
        }

        let Some(camera_entity) = camera
            .map(UiTargetCamera::entity)
            .or(default_ui_camera.get())
        else {
            continue;
        };

        let Ok(render_camera_entity) = mapping.get(camera_entity) else {
            continue;
        };

        // Extract a border box to display an outline for every UI Node in the layout
        extracted_uinodes.uinodes.push(ExtractedUiNode {
            // Add a large number to the UI node's stack index so that the overlay is always drawn on top
            stack_index: uinode.stack_index() + u32::MAX / 2,
            color: Hsla::sequential_dispersed(entity.index()).into(),
            rect: Rect {
                min: Vec2::ZERO,
                max: uinode.size(),
            },
            clip: maybe_clip
                .filter(|_| !debug_overlay.show_clipped)
                .map(|clip| clip.clip),
            image: AssetId::default(),
            extracted_camera_entity: render_camera_entity,
            item: ExtractedUiItem::Node {
                atlas_scaling: None,
                transform: transform.compute_matrix(),
                flip_x: false,
                flip_y: false,
                border: BorderRect::all(debug_overlay.line_width / uinode.inverse_scale_factor()),
                border_radius: uinode.border_radius(),
                node_type: NodeType::Border,
            },
            main_entity: entity.into(),
            render_entity: commands.spawn(TemporaryRenderEntity).id(),
        });
    }
}

#[derive(Default)]
pub struct UiDebugOverlayPlugin(pub UiDebugOverlay);

impl UiDebugOverlayPlugin {
    /// Start the app with the debug overlay disabled
    pub fn start_disabled() -> Self {
        Default::default()
    }

    /// Start the app with the debug overlay enabled
    pub fn start_enabled() -> Self {
        Self(UiDebugOverlay {
            enabled: true,
            ..Default::default()
        })
    }

    /// Set the debug overlay's line width in logical pixels
    pub fn with_line_width(mut self, line_width: f32) -> Self {
        self.0.line_width = line_width;
        self
    }
}

impl Plugin for UiDebugOverlayPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        #[cfg(feature = "bevy_reflect")]
        app.register_type::<UiDebugOverlay>();
        app.insert_resource(UiDebugOverlay { ..self.0 });

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(ExtractSchedule, extract_debug_overlay);
        }
    }
}

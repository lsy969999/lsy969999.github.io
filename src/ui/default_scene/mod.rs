use bevy::prelude::*;

use crate::asset::resource::BaseAssets;

use super::component::RootUiNode;

pub fn on_enter_default_scene(
    mut commands: Commands,
    q_root: Query<Entity, With<RootUiNode>>,
    base_asset: Res<BaseAssets>,
    // default_scene_asset: Res<DefaultSceneAssets>,
) {
    let Ok(root_ui) = q_root.get_single() else {
        return;
    };

    commands.entity(root_ui).with_children(|parent| {
        parent.spawn(Node { ..default() }).with_children(|parent| {
            parent.spawn((
                Text::new("default scene"),
                TextFont {
                    font: base_asset.font.clone(),
                    ..default()
                },
            ));
        });
    });
}

use bevy::prelude::*;

use crate::{app::state::MyAppState, asset::resource::BaseAssets};

use super::component::RootUiNode;
pub fn on_enter_dungeon_scene(
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
                Text::new("dungeon scene"),
                TextFont {
                    font: base_asset.font.clone(),
                    ..default()
                },
            ));
            parent
                .spawn((Button))
                .observe(
                    |out: Trigger<Pointer<Click>>,
                     mut next_state: ResMut<NextState<MyAppState>>| {
                        next_state.set(MyAppState::DefaultSceneAssetLoading);
                    },
                )
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("toggle"),
                        TextFont {
                            font: base_asset.font.clone(),
                            ..default()
                        },
                    ));
                });
        });
    });
}

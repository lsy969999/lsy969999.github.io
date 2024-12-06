use bevy::{color::palettes::css, prelude::*};

use crate::asset::resource::BaseAssets;

use super::component::RootUiNode;

pub mod system;

pub fn enter_base_loading_ui(mut commands: Commands, q_root: Query<Entity, With<RootUiNode>>) {
    // info!("enter_base_loading_ui");
    let Ok(root_ui) = q_root.get_single() else {
        info!("empty root_ui");
        return;
    };
    // info!("enter_base_loading_ui2");
    commands.entity(root_ui).with_children(|parent| {
        parent
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                // BackgroundColor(Color::from(css::ALICE_BLUE)),
            ))
            .with_children(|parent| {
                parent.spawn(Text::new("loading 1"));
            });
    });
}

pub fn enter_default_scene_loading_ui(
    mut commands: Commands,
    q_root: Query<Entity, With<RootUiNode>>,
    base_asset: Res<BaseAssets>,
) {
    let Ok(root_ui) = q_root.get_single() else {
        return;
    };

    commands.entity(root_ui).with_children(|parent| {
        parent
            .spawn(Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    Text::new("loading 2"),
                    TextFont {
                        font: base_asset.font.clone(),
                        ..default()
                    },
                ));
            });
    });
}

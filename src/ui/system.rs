use super::component::RootUiNode;
use bevy::prelude::*;
pub(super) fn setup_ui_root(mut commands: Commands) {
    info!("setup_ui_root");
    commands.spawn((
        Name::new("RootUiNode"),
        RootUiNode,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        PickingBehavior::IGNORE,
    ));
}

pub fn despawn_under_root_ui(
    mut commands: Commands,
    q_root: Query<(Entity, &Children), With<RootUiNode>>,
) {
    let Ok((_, children)) = q_root.get_single() else {
        return;
    };

    for &entity in children {
        commands.entity(entity).despawn_recursive();
    }
}

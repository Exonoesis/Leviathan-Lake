mod mock_game;

use bevy::prelude::*;
use cucumber::{given, then, when, World};
use helping_hand::{
    plugins::levels::MockLevelsPlugin,
    visuals::map::{ChangeLevel, TileType},
};
use mock_game::Game;

const MAX_NUM_ATTEMPTS: usize = 255;

#[given(regex = r"a Tiled map called (.+),")]
fn given_some_tiled_map(game: &mut Game, tiled_map_name: String) {
    game.add_plugin(MockLevelsPlugin);

    let map_path = format!("tests/test-assets/maps/{}", tiled_map_name);
    game.broadcast_event(ChangeLevel::new(&map_path));
}

#[when("the map is spawned,")]
fn when_map_spawned(game: &mut Game) {
    let expected_num_tiles_loaded = 21 * 21 * 2;
    for _i in 0..MAX_NUM_ATTEMPTS {
        game.tick();

        let has_map_loaded = game.get_number_of::<TileType>() == expected_num_tiles_loaded;
        if has_map_loaded {
            break;
        }
    }
}

#[then(regex = r"the player's x and y positions should be ([0-9]+)px, ([0-9]+)px.")]
fn verify_player_x_y_position(game: &mut Game, expected_player_x: f32, expected_player_y: f32) {
    let actual_player_position = game.get_player_position();

    let actual_player_x = actual_player_position.translation.x;
    let actual_player_y = actual_player_position.translation.y;

    assert_eq!(
        expected_player_x, actual_player_x,
        "x coordinates mismatching."
    );
    assert_eq!(
        expected_player_y, actual_player_y,
        "y coordinates mismatching."
    );
}

#[then(regex = r"the camera's x and y positions should be ([0-9]+)px, ([0-9]+)px.")]
fn verify_camera_x_y_position(game: &mut Game, expected_camera_x: f32, expected_camera_y: f32) {
    let actual_camera_position = game.get_of::<Transform, Camera2d>();
    let actual_camera_x = actual_camera_position.translation.x;
    let actual_camera_y = actual_camera_position.translation.y;

    assert_eq!(
        expected_camera_x, actual_camera_x,
        "x coordinates mismatching."
    );
    assert_eq!(
        expected_camera_y, actual_camera_y,
        "y coordinates mismatching."
    );
}

#[then(
    regex = r"the camera's position and player tile's center position are both ([0-9]+)px, ([0-9]+)px."
)]
fn verify_camera_centered_on_player_center(game: &mut Game, expected_x: f32, expected_y: f32) {
    let actual_camera_position = game.get_of::<Transform, Camera2d>();
    let actual_camera_x = actual_camera_position.translation.x;
    let actual_camera_y = actual_camera_position.translation.y;

    let actual_player_position = game.get_centered_player_position();
    let actual_player_x = actual_player_position.translation.x;
    let actual_player_y = actual_player_position.translation.y;

    assert_eq!(expected_x, actual_camera_x);
    assert_eq!(expected_y, actual_camera_y);
    assert_eq!(expected_x, actual_player_x);
    assert_eq!(expected_y, actual_player_y);
}

fn main() {
    futures::executor::block_on(Game::run("tests/feature-files/camera.feature"));
}

extern crate caper;

use caper::types::{RenderItemBuilder, TransformBuilder, MaterialBuilder, Vertex};
use caper::game::Game;
use caper::imgui::Ui;
use caper::input::Key;
use caper::utils::handle_fp_inputs;
use caper::mesh::DEF_NORMAL;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::new();

    let contour = vec![
        Vertex {
            position: [0f32, 0f32, 0f32],
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
        Vertex {
            position: [1f32, 0f32, 0f32],
            normal: DEF_NORMAL,
            texture: [1f32, 1f32],
        },
        Vertex {
            position: [1f32, 0f32, 1f32],
            normal: DEF_NORMAL,
            texture: [0f32, 1f32],
        },
        Vertex {
            position: [0f32, 0f32, 1f32],
            normal: DEF_NORMAL,
            texture: [1f32, 0f32],
        },
    ];

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(contour)
            .material(
                MaterialBuilder::default()
                    .shader_name("line".to_string())
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-0.5, 0.0, -5.0))
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );

    loop {
        // run the engine update
        game.update(|_: &Ui| {});

        // update the first person inputs
        handle_fp_inputs(&mut game.input, &mut game.cams[0]);

        // quit
        if game.input.keys_down.contains(&Key::Escape) {
            break;
        }
    }
}

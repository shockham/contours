extern crate caper;

use caper::types::{RenderItemBuilder, TransformBuilder, MaterialBuilder, DefaultTag};
use caper::game::{Game, Update, RenderItems, UpdateStatus, start_loop};
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_perlin_mesh;
use caper::utils::handle_fp_inputs;

mod shaders;

fn main() {
    // crate an instance of the game struct
    let (mut game, event_loop) = Game::<DefaultTag>::new();

    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_perlin_mesh((0f32, 0f32), 100f32))
            .material(
                MaterialBuilder::default()
                    .shader_name("contours".to_string())
                    .build()
                    .unwrap(),
            )
            .instance_transforms(vec![
                TransformBuilder::default()
                    .pos((-50f32, 0f32, -50f32))
                    .cull(false)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap(),
    );

    // initial setup
    {
        shaders::add_custom_shaders(&mut game);
    }

    // run the engine update
    start_loop(event_loop, move |events| {
        game.update(
            |_: &Ui| {},
            |game: &mut Game<DefaultTag>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut game.input, &mut game.cams[0]);

                // quit
                if game.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
            events,
        )
    });
}

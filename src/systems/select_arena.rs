use amethyst::{
    ecs::prelude::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
    ui::UiTransform,
};

pub struct SelectArena;

impl<'s> System<'s> for SelectArena {
    type SystemData = (
        WriteStorage<'s, UiTransform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut ui_transforms, input): Self::SystemData) {
        for ui_transform in (&mut ui_transforms).join() {
            if let Some(movement) = input.axis_value("vertical") {
                ui_transform.local_y += 30.0 * movement; // TODO use font size
            };
        }
    }
}

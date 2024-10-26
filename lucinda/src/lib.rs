mod board;

use godot::classes::{GridContainer, IGridContainer, Panel};
use godot::prelude::*;
use crate::board::BoardBuilder;


#[gdextension]
unsafe impl ExtensionLibrary for LucindaGrid {}

#[derive(GodotClass)]
#[class(base = GridContainer)]
struct LucindaGrid {
    board: board::ValidBoard,
    slot_instances: Vec<Gd<Panel>>,
    base: Base<GridContainer>
}

#[godot_api]
impl IGridContainer for LucindaGrid {
    fn init(base: Base<GridContainer>) -> Self {
        let board = loop {
            if let Some(board) = BoardBuilder::new()
                .place_queens()
                .flood_fill()
                .validate_unique() {
                break board;
            }
        };

        Self {
            board,
            slot_instances: Vec::new(),
            base
        }
    }

    fn ready(&mut self) {
        let scene : Gd<PackedScene> = load("res://slot.tscn");

        self.slot_instances.extend(self.board.iter().map(|x| {
            let mut y = scene.instantiate_as::<Panel>();

            let args = [x.region.as_color().to_variant()];

            y.call("setColour".into(), &args);

            y
        }));

        let instances_to_child = self.slot_instances.clone();
        for child in instances_to_child {
            self.base_mut().add_child(child);
        }
    }
}



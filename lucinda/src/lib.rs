mod board;

use std::env::args;
use godot::classes::{GridContainer, IGridContainer, Panel};
use godot::classes::resource_loader::ExLoad;
use godot::prelude::*;
use godot::sys::InitLevel::Scene;
use nalgebra::SMatrix;
use crate::board::BoardBuilder;

struct Lucinda;

#[gdextension]
unsafe impl ExtensionLibrary for Lucinda {
}

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
        let scene = load("res://slot.tscn");

        self.slot_instances.extend(self.board.iter().map(|x| {
            let mut y = scene.instantiate_as::<Panel>();

            let args = [x.region.as_color().to_variant()];

            y.call("set_color".into(), &args);

            y
        }));

    }
}



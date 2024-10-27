mod board;

use godot::classes::{GridContainer, IGridContainer, Panel};
use godot::prelude::*;
use nalgebra::SMatrix;
use crate::board::{validate_grid, BoardBuilder};


#[gdextension]
unsafe impl ExtensionLibrary for LucindaGrid {}

#[derive(GodotClass)]
#[class(base = GridContainer)]
struct LucindaGrid {
    #[export]
    seed: GString,
    board: board::ValidBoard,
    slot_instances: Option<SMatrix<Gd<Panel>, 8, 8>>,
    base: Base<GridContainer>
}

#[godot_api]
impl LucindaGrid {
    #[func]
    fn regenerate(&mut self) {
        self.board = loop {
            let hash = self.seed.hash();
            let seed = [
                (hash << (u8::BITS * 3)) as u8,
                (hash << (u8::BITS * 2)) as u8,
                (hash << u8::BITS) as u8,
                hash as u8
            ];
            
            let mut seed_seed : [u8; 32] = [0;32];
            
            for i in 0..(32 / 4) {
                seed_seed[i..(i * 4)].copy_from_slice(&seed);
            }
            
            if let Some(board) = BoardBuilder::new_with_seed(seed_seed)
                .place_queens()
                .flood_fill()
                .validate_unique() {
                break board;
            }
        };

        for (child, slot) in self.slot_instances.as_mut().unwrap().iter_mut().zip(self.board.iter()) {
            let args = [slot.region.as_color().to_variant()];
            child.call("resetSlot".into(), &[]);
            child.call("setColour".into(), &args);
        }
    }

    #[func]
    fn check(&mut self) {
        let internal_slot_instance_ref = self.slot_instances
            .as_mut()
            .unwrap();

        let user_queens: SMatrix<bool, 8, 8> = internal_slot_instance_ref
            .map(|mut slot| { slot.call("isQueen".into(), &[]).booleanize() });
        let validated = validate_grid(*self.board, user_queens);

        for (x, y) in internal_slot_instance_ref.iter_mut().zip(validated.iter()) {
            let args = [(!*y).to_variant()];
            x.call("setValidity".into(), &args);
        }
    }
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
            slot_instances: None,
            base,
            seed: "Queendoms".into()
        }
    }

    fn ready(&mut self) {
        let scene : Gd<PackedScene> = load("res://slot.tscn");

        self.slot_instances = Some(self.board.map(|x| {
            let mut y = scene.instantiate_as::<Panel>();

            let args = [x.region.as_color().to_variant()];

            y.call("setColour".into(), &args);

            y
        }));

        for child in self.slot_instances
            .clone()
            .expect("Slot instances are explicitly initialized")
            .into_iter() {
            self.base_mut().add_child(child);
        }
    }
}

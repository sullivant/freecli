use serde::{Deserialize, Serialize};

use crate::cli::AppArgs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LocationType {
    Column,
    Freecell,
    Foundation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: LocationType,
    pub from_idx: usize,
    pub to: LocationType,
    pub to_idx: usize,
}

impl Move {
    pub fn from_args( args: &AppArgs ) -> Result<Option<Self>, String> {
        let mut flags = Vec::new(); // Our flags as set.

        for (i, &set) in [
            args.c0, args.c1, args.c2, args.c3, args.c4, args.c5, args.c6, args.c7,
        ].iter().enumerate() {
            if set {
                flags.push((LocationType::Column, i, format!("--c{}",i)));
            }
        }

        for (i, &set) in [
            args.f0, args.f1, args.f2, args.f3,
        ].iter().enumerate() {
            if set {
                flags.push((LocationType::Freecell, i, format!("--f{}",i)));
            }
        }

        if args.foundation {
            flags.push((LocationType::Foundation, 0, "--foundation".to_string()));
        }

        // Figure out the right order now.
        match flags.len() {
            0 => Ok(None), // Not a move
            1 => Err(format!("Only one move location supplied ({}), need two.",flags[0].2)),
            2 => {
                let (from_ty, from_idx, _) = &flags[0];
                let (to_ty, to_idx, _) = &flags[1];

                Ok(Some(Move {
                    from: from_ty.clone(), from_idx: *from_idx,
                    to: to_ty.clone(), to_idx: *to_idx
                }))
            },
            _ => {
                let names = flags.iter().map(|f| &f.2).cloned().collect::<Vec<_>>();
                Err(format!("Too many locations provided: {}", names.join(", ")))
            }
        }
    }
}


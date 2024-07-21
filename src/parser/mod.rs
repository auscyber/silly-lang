use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub silly,
    "/parser/silly.rs"
);
pub mod ast;

use clap::{Parser, clap_derive::ArgEnum, Args};

use crate::preset_scenes::PresetScene;

#[derive(Parser)]
pub struct CliArguments {
    #[clap(short='s', long="samples")]
    pub num_samples: u32,
    #[clap(short='m', long="multithreaded")]
    pub multithreaded: bool,
    #[clap(flatten)]
    pub multithreaded_settings: MultithreadedSettings,
    #[clap(short='o', long="output")]
    pub output_file: Option<String>,
    #[clap(long="scene", arg_enum, value_parser, default_value_t=PresetScene::JumpingBalls)]
    pub preset_scene: PresetScene,
}

#[derive(Debug, Args)]
pub struct MultithreadedSettings {
    #[clap(value_parser, short='i', long="interactive")]
    pub interactive: bool,
    #[clap(long="strategy", arg_enum, value_parser, default_value_t=RenderStrategy::TileAverage)]
    pub render_strategy: RenderStrategy,
    #[clap(long="tile-size", default_value_t=64)]
    pub tile_size: u32
}

#[derive(Debug, Clone, ArgEnum)]
pub enum RenderStrategy {
    ProgressiveAverage,
    TileFull,
    TileAverage
}
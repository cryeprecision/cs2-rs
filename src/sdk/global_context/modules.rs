use anyhow::Context;
use windows::core::{s, PCSTR};

use crate::oof::module::Module;

// https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/common.h#L13
const CLIENT_DLL: PCSTR = s!("client.dll");
const DBGHELP_DLL: PCSTR = s!("dbghelp.dll");
const ENGINE2_DLL: PCSTR = s!("engine2.dll");
const GAMEOVERLAYRENDERER_DLL: PCSTR = s!("GameOverlayRenderer64.dll");
const INPUTSYSTEM_DLL: PCSTR = s!("inputsystem.dll");
const LOCALIZE_DLL: PCSTR = s!("localize.dll");
const MATCHMAKING_DLL: PCSTR = s!("matchmaking.dll");
const MATERIAL_SYSTEM2_DLL: PCSTR = s!("materialsystem2.dll");
const NAVSYSTEM_DLL: PCSTR = s!("navsystem.dll");
const PARTICLES_DLL: PCSTR = s!("particles.dll");
const RENDERSYSTEM_DLL: PCSTR = s!("rendersystemdx11.dll");
const RESOURCESYSTEM_DLL: PCSTR = s!("resourcesystem.dll");
const SCENESYSTEM_DLL: PCSTR = s!("scenesystem.dll");
const SCHEMASYSTEM_DLL: PCSTR = s!("schemasystem.dll");
const SDL3_DLL: PCSTR = s!("SDL3.dll");
const TIER0_DLL: PCSTR = s!("tier0.dll");

#[derive(Debug)]
pub struct Modules {
    pub client: Module,
    pub dbg_help: Module,
    pub engine2: Module,
    pub game_overlay_renderer: Module,
    pub input_system: Module,
    pub localize: Module,
    pub matchmaking: Module,
    pub material_system: Module,
    pub nav_system: Module,
    pub particles: Module,
    pub render_system: Module,
    pub resource_system: Module,
    pub scene_system: Module,
    pub schema_system: Module,
    pub sdl3: Module,
    pub tier0: Module,
}

impl Modules {
    pub unsafe fn new() -> anyhow::Result<Modules> {
        Ok(Modules {
            client: Module::new(CLIENT_DLL).context("find client module")?,
            dbg_help: Module::new(DBGHELP_DLL).context("find dbg_help module")?,
            engine2: Module::new(ENGINE2_DLL).context("find engine module")?,
            game_overlay_renderer: Module::new(GAMEOVERLAYRENDERER_DLL)
                .context("find game_overlay_renderer module")?,
            input_system: Module::new(INPUTSYSTEM_DLL).context("find input_system module")?,
            localize: Module::new(LOCALIZE_DLL).context("find localize module")?,
            matchmaking: Module::new(MATCHMAKING_DLL).context("find matchmaking module")?,
            material_system: Module::new(MATERIAL_SYSTEM2_DLL)
                .context("find material_system module")?,
            nav_system: Module::new(NAVSYSTEM_DLL).context("find nav_system module")?,
            particles: Module::new(PARTICLES_DLL).context("find particles module")?,
            render_system: Module::new(RENDERSYSTEM_DLL).context("find render_system module")?,
            resource_system: Module::new(RESOURCESYSTEM_DLL)
                .context("find resource_system module")?,
            scene_system: Module::new(SCENESYSTEM_DLL).context("find scenesystem module")?,
            schema_system: Module::new(SCHEMASYSTEM_DLL).context("find schema_system module")?,
            sdl3: Module::new(SDL3_DLL).context("find sdl3 module")?,
            tier0: Module::new(TIER0_DLL).context("find tier0 module")?,
        })
    }
}

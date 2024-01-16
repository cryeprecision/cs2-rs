use anyhow::Context;

use super::modules::Modules;
use crate::oof::module::Module;
use crate::oof::pattern::Pattern;

unsafe fn find_pattern(pattern: &str, module: &Module) -> anyhow::Result<*mut u8> {
    pattern
        .parse::<Pattern>()
        .context("parse pattern")?
        .find_in(module.module_slice())
        .map(|offset| module.module_base().add(offset) as _)
        .context("find pattern in module")
}

mod client_dll {
    use anyhow::Context;

    use super::super::modules::Modules;
    use super::find_pattern;

    const LEVEL_INIT: &str = "48 89 5C 24 ? 56 48 83 EC ? 48 8B 0D ? ? ? ? 48 8B F2";
    const LEVEL_SHUTDOWN: &str = "48 83 EC ? 48 8B 0D ? ? ? ? 48 8D 15 ? ? ? ? 45 33 C9 45 33 C0 \
                                  48 8B 01 FF 50 ? 48 85 C0 74 ? 48 8B 0D ? ? ? ? 48 8B D0 4C 8B \
                                  01 48 83 C4 ? 49 FF 60 ? 48 83 C4 ? C3 CC CC CC 48 83 EC ? 4C \
                                  8B D9";
    const CSGO_INPUT: &str = "48 8B 0D ? ? ? ? 48 8B 01 FF 50 ? 8B DF";
    const GLOBAL_VARS: &str = "48 89 0D ? ? ? ? 48 89 41";
    const GET_BASE_ENTITY: &str = "81 FA ? ? ? ? 77 ? 8B C2 C1 F8 ? 83 F8 ? 77 ? 48 98 48 8B 4C \
                                   C1 ? 48 85 C9 74 ? 8B C2 25 ? ? ? ? 48 6B C0 ? 48 03 C8 74 ? \
                                   8B 41 ? 25 ? ? ? ? 3B C2 75 ? 48 8B 01";
    const SET_VIEW_ANGLES: &str = "F2 41 0F 10 00 4C 63 CA";
    const GET_MATRIX_FOR_VIEW: &str = "40 53 48 81 EC ? ? ? ? 49 8B C1";

    #[derive(Debug)]
    pub struct Patterns {
        pub level_init: *mut u8,
        pub level_shutdown: *mut u8,
        pub csgo_input: *mut u8,
        pub global_vars: *mut u8,
        pub get_base_entity: *mut u8,
        pub set_view_angles: *mut u8,
        pub get_matrix_for_view: *mut u8,
    }

    impl Patterns {
        pub unsafe fn new(modules: &Modules) -> anyhow::Result<Patterns> {
            Ok(Patterns {
                level_init: find_pattern(LEVEL_INIT, &modules.client)
                    .context("find LEVEL_INIT pattern")?,
                level_shutdown: find_pattern(LEVEL_SHUTDOWN, &modules.client)
                    .context("find LEVEL_SHUTDOWN pattern")?,
                csgo_input: find_pattern(CSGO_INPUT, &modules.client)
                    .context("find CSGO_INPUT pattern")?,
                global_vars: find_pattern(GLOBAL_VARS, &modules.client)
                    .context("find GLOBAL_VARS pattern")?,
                get_base_entity: find_pattern(GET_BASE_ENTITY, &modules.client)
                    .context("find GET_BASE_ENTITY pattern")?,
                set_view_angles: find_pattern(SET_VIEW_ANGLES, &modules.client)
                    .context("find SET_VIEW_ANGLES pattern")?,
                get_matrix_for_view: find_pattern(GET_MATRIX_FOR_VIEW, &modules.client)
                    .context("find GET_MATRIX_FOR_VIEW pattern")?,
            })
        }
    }
}

mod particles_dll {
    use anyhow::Context;

    use super::super::modules::Modules;
    use super::find_pattern;

    const FIND_KEY_VAR: &str = "48 89 5C 24 ? 57 48 81 EC ? ? ? ? 33 C0 8B DA";
    const SET_MATERIAL_SHADER_TYPE: &str =
        "48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 83 EC ? 0F B6 01 45 0F B6 F9 8B \
         2A 4D 8B E0 4C 8B 72 ? 48 8B F9 C0 E8 ? 24 ? 3C ? 74 ? 41 B0 ? B2 ? E8 ? ? ? ? 0F B6 07 \
         33 DB C0 E8 ? 24 ? 3C ? 75 ? 48 8B 77 ? EB ? 48 8B F3 4C 8D 44 24 ? C7 44 24 ? ? ? ? ? \
         48 8D 54 24 ? 89 6C 24 ? 48 8B CE 4C 89 74 24 ? E8 ? ? ? ? 8B D0 83 F8 ? 75 ? 45 33 C9 \
         89 6C 24 ? 4C 8D 44 24 ? 4C 89 74 24 ? 48 8B D7 48 8B CE E8 ? ? ? ? 8B D0 0F B6 0F C0 E9 \
         ? 80 E1 ? 80 F9 ? 75 ? 48 8B 4F ? EB ? 48 8B CB 8B 41 ? 85 C0 74 ? 48 8D 59 ? 83 F8 ? 76 \
         ? 48 8B 1B 48 63 C2 4D 85 E4";
    const SET_MATERIAL: &str = "48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 83 EC ? 0F \
                                B6 01 45 0F B6 F9 8B 2A 48 8B F9";

    #[derive(Debug)]
    pub struct Patterns {
        pub find_key_var: *mut u8,
        pub set_material_shader_type: *mut u8,
        pub set_material: *mut u8,
    }

    impl Patterns {
        pub unsafe fn new(modules: &Modules) -> anyhow::Result<Patterns> {
            Ok(Patterns {
                find_key_var: find_pattern(FIND_KEY_VAR, &modules.particles)
                    .context("find FIND_KEY_VAR pattern")?,
                set_material_shader_type: find_pattern(
                    SET_MATERIAL_SHADER_TYPE,
                    &modules.particles,
                )
                .context("find SET_MATERIAL_SHADER_TYPE pattern")?,
                set_material: find_pattern(SET_MATERIAL, &modules.particles)
                    .context("find SET_MATERIAL pattern")?,
            })
        }
    }
}

mod render_system_dll {
    use anyhow::Context;

    use super::super::modules::Modules;
    use super::find_pattern;

    const SWAP_CHAIN: &str = "66 0F 7F 05 ? ? ? ? 66 0F 7F 0D ? ? ? ? 48 89 35";

    #[derive(Debug)]
    pub struct Patterns {
        pub swap_chain: *mut u8,
    }

    impl Patterns {
        pub unsafe fn new(modules: &Modules) -> anyhow::Result<Patterns> {
            Ok(Patterns {
                swap_chain: find_pattern(SWAP_CHAIN, &modules.render_system)
                    .context("find SWAP_CHAIN pattern")?,
            })
        }
    }
}

mod scene_system_dll {
    use anyhow::Context;

    use super::super::modules::Modules;
    use super::find_pattern;

    const DRAW_OBJECT: &str = "48 8B C4 48 89 50 ? 55 41 56";

    #[derive(Debug)]
    pub struct Patterns {
        pub draw_object: *mut u8,
    }

    impl Patterns {
        pub unsafe fn new(modules: &Modules) -> anyhow::Result<Patterns> {
            Ok(Patterns {
                draw_object: find_pattern(DRAW_OBJECT, &modules.scene_system)
                    .context("find DRAW_OBJECT pattern")?,
            })
        }
    }
}

mod material_system2_dll {
    use anyhow::Context;

    use super::super::modules::Modules;
    use super::find_pattern;

    const CREATE_MATERIAL: &str =
        "48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 56 48 81 EC ? ? ? ? 48 8D 0D";

    #[derive(Debug)]
    pub struct Patterns {
        pub crate_material: *mut u8,
    }

    impl Patterns {
        pub unsafe fn new(modules: &Modules) -> anyhow::Result<Patterns> {
            Ok(Patterns {
                crate_material: find_pattern(CREATE_MATERIAL, &modules.material_system)
                    .context("find CREATE_MATERIAL pattern")?,
            })
        }
    }
}

#[derive(Debug)]
pub struct Patterns {
    particles_dll: particles_dll::Patterns,
    render_system_dll: render_system_dll::Patterns,
    scene_system_dll: scene_system_dll::Patterns,
    material_system2_dll: material_system2_dll::Patterns,
}

impl Patterns {
    pub unsafe fn new(modules: &Modules) -> anyhow::Result<Patterns> {
        Ok(Patterns {
            particles_dll: particles_dll::Patterns::new(modules)
                .context("find particles_dll patterns")?,
            render_system_dll: render_system_dll::Patterns::new(modules)
                .context("find render_system_dll patterns")?,
            scene_system_dll: scene_system_dll::Patterns::new(modules)
                .context("find scene_system_dll patterns")?,
            material_system2_dll: material_system2_dll::Patterns::new(modules)
                .context("find material_system2_dll patterns")?,
        })
    }
}

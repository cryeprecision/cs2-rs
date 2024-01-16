use anyhow::Context;
use windows::core::s;

use super::modules::Modules;
use crate::sdk::interfaces::engine_client::EngineClient;
use crate::sdk::interfaces::mem_alloc::MemAlloc;
use crate::sdk::interfaces::InterfaceRegister;

#[derive(Debug)]
pub struct Interfaces {
    pub engine_client: EngineClient,
    pub mem_alloc: MemAlloc,
}

impl Interfaces {
    pub unsafe fn new(modules: &Modules) -> anyhow::Result<Interfaces> {
        let engine_client = EngineClient::new(
            InterfaceRegister::capture_interface(&modules.engine2, "Source2EngineToClient001")
                .context("get engine_client pointer")?,
        );
        let mem_alloc = MemAlloc::new(
            modules
                .tier0
                .get_export(s!("g_pMemAlloc"))
                .context("get mem_alloc pointer")?
                .cast::<*mut u8>()
                .read(),
        );

        Ok(Interfaces {
            engine_client,
            mem_alloc,
        })
    }
}

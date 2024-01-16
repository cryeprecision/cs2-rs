use anyhow::Context;

mod interfaces;
mod modules;
mod patterns;

#[derive(Debug)]
pub struct GlobalContext {
    pub modules: modules::Modules,
    pub patterns: patterns::Patterns,
    pub interfaces: interfaces::Interfaces,
}

impl GlobalContext {
    pub unsafe fn new() -> anyhow::Result<GlobalContext> {
        let modules = modules::Modules::new().context("find required modules")?;
        let patterns = patterns::Patterns::new(&modules).context("find required patterns")?;
        let interfaces =
            interfaces::Interfaces::new(&modules).context("find required interfaces")?;

        Ok(GlobalContext {
            modules,
            patterns,
            interfaces,
        })
    }
}

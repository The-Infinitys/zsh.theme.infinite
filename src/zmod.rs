use zsh_module::{Module, ModuleBuilder};

pub fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    let module = ModuleBuilder::new(ZshInfiniteModule).build();
    Ok(module)
}

struct ZshInfiniteModule;

impl ZshInfiniteModule {}

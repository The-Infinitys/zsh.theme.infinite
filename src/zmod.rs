use zsh_system::{Features, ZshModule};
#[derive(Default)]
struct ZshInfinite {}

impl ZshModule for ZshInfinite {
    fn setup(&mut self) -> i32 {
        0
    }
    fn boot(&mut self) -> i32 {
        0
    }
    fn features(&self) -> Features {
        Features::new()
    }

    fn cleanup(&mut self) -> i32 {
        0
    }
    fn finish(&mut self) -> i32 {
        0
    }
}

zsh_system::export_module!(ZshInfinite);

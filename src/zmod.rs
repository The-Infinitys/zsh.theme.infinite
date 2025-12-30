use crate::args::PromptType;
use crate::zsh;
use once_cell::sync::OnceCell;
use std::ffi::{CString, c_void};
use std::os::raw::c_int; // c_char を削除
// getiparam を削除
use zsh_module::{Builtin, MaybeError, Module, ModuleBuilder, Opts};
use zsh_sys::{addhookfunc, hookdef, setsparam};

static TOKIO_RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();

// --- プロンプト生成ロジック ---

async fn left_prompt() -> String {
    zsh::build_prompt(&PromptType::Left).await.build()
}

async fn right_prompt() -> String {
    zsh::build_prompt(&PromptType::Right).await.build()
}

// --- フック関数本体 ---

unsafe extern "C" fn rust_precmd_hook(_: *mut hookdef, _: *mut c_void) -> c_int {
    // 動作確認用のログ（stderrに出すことでzshの出力を汚しにくい）
    eprintln!("[DEBUG] rust_precmd_hook called");

    if let Some(rt) = TOKIO_RUNTIME.get() {
        let left = rt.block_on(left_prompt());
        let right = rt.block_on(right_prompt());

        unsafe {
            set_zsh_string("PROMPT", &left);
            set_zsh_string("RPROMPT", &right);
        }
    }
    0
}
unsafe fn set_zsh_string(name: &str, value: &str) {
    let name_c = CString::new(name).unwrap();
    let value_c = CString::new(value).unwrap();
    // 実際の外部関数呼び出しを unsafe ブロックで囲む
    unsafe {
        setsparam(name_c.as_ptr() as *mut _, value_c.as_ptr() as *mut _);
    }
}

// --- モジュール定義 ---

zsh_module::export_module!(zsh_infinite, setup);

struct ZshInfiniteModule;

impl ZshInfiniteModule {
    fn init(&mut self, _name: &str, _args: &[&str], _opts: Opts) -> MaybeError {
        // Tokio Runtime の初期化
        eprintln!("[DEBUG] zsh-infinite-module command executed");
        TOKIO_RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
        });

        // precmd フックの登録
        unsafe {
            let hook_name = CString::new("ahostick").unwrap();
            let ret = addhookfunc(hook_name.as_ptr() as *mut _, Some(rust_precmd_hook));
            eprintln!("[DEBUG] addhookfunc ret: {}", ret);
        }
        Ok(())
    }
}

fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    let module = ModuleBuilder::new(ZshInfiniteModule)
        .builtin(ZshInfiniteModule::init, Builtin::new("zsh-infinite-module"))
        .build();
    Ok(module)
}

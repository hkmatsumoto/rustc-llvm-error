#![feature(rustc_private)]

extern crate rustc_codegen_llvm;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate rustc_llvm;
extern crate rustc_session;
extern crate rustc_span;

use std::{path::PathBuf, process::Command};

use rustc_codegen_llvm::llvm::*;

fn config() -> rustc_interface::Config {
    let sysroot = Command::new("rustc")
        .arg("--print=sysroot")
        .output()
        .unwrap();
    let sysroot = std::str::from_utf8(&sysroot.stdout).unwrap().trim();

    rustc_interface::Config {
        opts: rustc_session::config::Options {
            maybe_sysroot: Some(PathBuf::from(sysroot)),
            ..Default::default()
        },
        crate_cfg: Default::default(),
        crate_check_cfg: Default::default(),
        input: rustc_session::config::Input::Str {
            name: rustc_span::FileName::Anon(0),
            input: "".to_owned(),
        },
        input_path: Default::default(),
        output_file: Default::default(),
        output_dir: Default::default(),
        file_loader: Default::default(),
        diagnostic_output: rustc_session::DiagnosticOutput::Default,
        lint_caps: Default::default(),
        parse_sess_created: Default::default(),
        register_lints: Default::default(),
        override_queries: Default::default(),
        make_codegen_backend: Default::default(),
        registry: rustc_errors::registry::Registry::new(&[]),
    }
}

fn main() {
    rustc_llvm::initialize_available_targets();

    rustc_interface::run_compiler(config(), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().peek_mut().enter(|_| unsafe {
                let llcx = LLVMRustContextCreate(false);
                LLVMContextDispose(llcx);
            })
        })
    })
}

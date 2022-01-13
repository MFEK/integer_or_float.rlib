#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate cargo_emit;
#[allow(unused_imports)] // used in panic_if_no_std_and_compiling_tests!()
#[macro_use]
extern crate indoc;

#[allow(dead_code)] // used in panic_if_no_std_and_compiling_tests!()
fn compiling_tests() -> bool {
    option_env!("CARGO_PRIMARY_PACKAGE").is_none()
        && option_env!("CARGO_TEMPDIR_PATH").is_some()
}

macro_rules! panic_if_no_std_and_compiling_tests {
    () => {
        cfg_if! {
            if #[cfg(all(feature = "no_std", not(feature = "ignore_no_std_tests_check")))] {
                compile_error!(indoc!{"
                    Tests will run, but *will* link `std`! Therefore, if you are trying to test \
                    that your code *works* in no_std rather than just compiles, `cargo test` \
                    can't tell you that!
                "});
            }
        }
    }
}

fn main() {
    panic_if_no_std_and_compiling_tests!();
    if cfg!(feature = "no_std") {
        rustc_cfg!("no_std");
    } else {
        rustc_cfg!("std");
    }
    if cfg!(any(feature = "serde", feature = "default")) {
        rustc_cfg!("use_serde");
    }
    if cfg!(feature = "hash") {
        rustc_cfg!("with_impl_hash");
    }
}

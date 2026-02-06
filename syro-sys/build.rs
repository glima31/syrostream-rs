const SYRO_SRC: &str = "volcasample/syro";

fn main() {
    cc::Build::new()
        .std("c11")
        .file(format!("{SYRO_SRC}/korg_syro_volcasample.c"))
        .file(format!("{SYRO_SRC}/korg_syro_func.c"))
        .file(format!("{SYRO_SRC}/korg_syro_comp.c"))
        .include(SYRO_SRC)
        .compile("syro");
}

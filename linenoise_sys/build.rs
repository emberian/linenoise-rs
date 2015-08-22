extern crate gcc;


fn main() {
    gcc::compile_library("liblinenoise.a", &["linenoise.c"]);
}

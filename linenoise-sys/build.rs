extern crate gcc;


fn main() {
    gcc::compile_library("liblinenoise.a", &std::default::Default::default(), &["linenoise.c"]);
}

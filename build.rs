extern crate tonic_build;

fn main() {
    tonic_build::compile_protos("proto/helloworld.proto").unwrap();
}

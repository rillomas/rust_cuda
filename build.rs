use std::process::Command;
use std::env;
use std::path::Path;
use std::str::from_utf8;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // build cuda device library
    let compile_out = format!("{}/hello.o", out_dir);
    let nvcc_out = Command::new("nvcc")
                        .args(&["hello/hello.cu", "-arch=sm_30", "-Xcompiler", "-fPIC", "-dc", "-o"])
                        .arg(&compile_out)
                        .output().unwrap().stderr;
    let nvcc_out_str = from_utf8(&nvcc_out).unwrap();
    println!("nvcc compile err: {}", nvcc_out_str);
    let dlink_out = format!("{}/hello_dev.o", out_dir);
    let nvcc_out = Command::new("nvcc")
                        .args(&["-arch=sm_30","-dlink", &compile_out, "-Xcompiler", "-fPIC", "-o"])
                        .arg(&dlink_out)
                        .output().unwrap().stderr;
    let nvcc_out_str = from_utf8(&nvcc_out).unwrap();
    println!("nvcc link err: {}", nvcc_out_str);

    let ar_out = Command::new("ar")
                      .args(&["crus", "libhello_dev.a", "hello_dev.o"])
                      //.args(&["crus", "libhello.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .output().unwrap().stderr;
    let ar_out_str = from_utf8(&ar_out).unwrap();
    println!("ar dev err: {}", ar_out_str);

    // build cuda host library
    let ar_out = Command::new("ar")
                      .args(&["crus", "libhello_host.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .output().unwrap().stderr;
    let ar_out_str = from_utf8(&ar_out).unwrap();
    println!("ar host err: {}", ar_out_str);

    // finalize
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-search=native=/usr/local/cuda-6.5/targets/x86_64-linux/lib/");
    println!("cargo:rustc-link-lib=static=hello_dev");
    println!("cargo:rustc-link-lib=static=hello_host");
    println!("cargo:rustc-link-lib=dylib=cudart");
    //panic!("Nope");
}

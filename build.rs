fn main() {
    println!("cargo:rerun-if-changed=src/gui/ui.slint");
    println!("cargo:rerun-if-changed=src/gui/knob.slint");
    println!("cargo:rerun-if-changed=src/gui/segmented.slint");
    println!("cargo:rerun-if-changed=src/gui/lfo_icon.slint");
    slint_build::compile("src/gui/ui.slint").unwrap();
}

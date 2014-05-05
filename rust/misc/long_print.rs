pub fn general() {
    println("Usage: rustpkg [options] <cmd> [args..]

Where <cmd> is one of:
    build, clean, do, info, install, list, prefer, test, uninstall, unprefer

Options:

    -h, --help                  Display this message
    --sysroot PATH              Override the system root
    <cmd> -h, <cmd> --help      Display help for <cmd>");
}

pub fn build() {
    println("rustpkg build [options..] [package-ID]

Build the given package ID if specified. With no package ID argument,
build the package in the current directory. In that case, the current
directory must be a direct child of an `src` directory in a workspace.

Options:
    -c, --cfg      Pass a cfg flag to the package script
    --no-link      Compile and assemble, but don't link (like -c in rustc)
    --no-trans     Parse and translate, but don't generate any code
    --pretty       Pretty-print the code, but don't generate output
    --parse-only   Parse the code, but don't typecheck or generate code
    -S             Generate assembly code, but don't assemble or link it
    -S --emit-llvm Generate LLVM assembly code
    --emit-llvm    Generate LLVM bitcode
    --linker PATH  Use a linker other than the system linker
    --link-args [ARG..] Extra arguments to pass to the linker
    --opt-level=n  Set the optimization level (0 <= n <= 3)
    -O             Equivalent to --opt-level=2
    --save-temps   Don't delete temporary files
    --target TRIPLE Set the target triple
    --target-cpu CPU Set the target CPU
    -Z FLAG        Enable an experimental rustc feature (see `rustc --help`)");
}

fn main() {
	general();
	build();
}

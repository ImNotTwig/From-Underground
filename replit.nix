{ pkgs }: {
	deps = [
		pkgs.rustc
		pkgs.rustfmt
		pkgs.rustup
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}
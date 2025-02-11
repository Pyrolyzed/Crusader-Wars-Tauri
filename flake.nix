{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }@inputs:
     let
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
      in
      {
        devShells.x86_64-linux.default = pkgs.mkShell {
          buildInputs = with pkgs; [
	    at-spi2-atk
	    atkmm
	    cairo
	    gdk-pixbuf
	    glib
	    gtk3
	    harfbuzz
	    librsvg
	    libsoup_3
	    pango
	    webkitgtk_4_1
	    openssl
	  ];

	  nativeBuildInputs = with pkgs; [
	    pkg-config
	    gobject-introspection
	    cargo
	    nodejs
	    rustc
	  ];

	  shellHook = ''
            XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS";
	  '';
        };
      };
}

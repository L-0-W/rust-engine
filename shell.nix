{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
  ];

  buildInputs = with pkgs; [
    udev
    alsa-lib
    vulkan-loader
    # Dependências de Janela (X11 e Wayland)
    libxkbcommon
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];


  # Define a variável de ambiente para que o driver de vídeo seja encontrado
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
      pkgs.lib.makeLibraryPath (with pkgs; [
        vulkan-loader
        libxkbcommon
        wayland
      ])
    }"
    echo "Preparado para rodar WGPU"
  '';

}

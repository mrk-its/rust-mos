#!/bin/bash
if test "$CARGO_CFG_TARGET_ARCH" = "aarch64"; then
  export PKG_CONFIG_SYSROOT_DIR=${ARM64_SYSROOT}
  export PKG_CONFIG_PATH=
  export PKG_CONFIG_LIBDIR=${ARM64_SYSROOT}/usr/lib/aarch64-linux-gnu/pkgconfig
  # for i in "$@"; do
  #   if test "$i" = "--cflags" -o "$i" = "--cflags-only-I"; then
  #     echo -n "-I${ARM64_SYSROOT}/usr/include/aarch64-linux-gnu ";
  #   fi
  # done
fi
exec /usr/bin/pkg-config "$@"

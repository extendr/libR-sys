#!/usr/bin/env bash
# Print the sorted C symbols exported by the installed R shared library.
#
# libR-sys ships hand-maintained, per-version bindings (in-tree bindgen was
# removed in #250). Nothing automatically notices when R's C API surface
# shifts, so bindings silently go stale. `check-r-api.yml` snapshots this list
# and opens a PR when it changes, as an early warning that the bindings may
# need updating. No bindgen/clang required.
set -euo pipefail

r_home="$(R RHOME)"
case "$(uname -s)" in
  Darwin) lib="$r_home/lib/libR.dylib"; nm_args=(-g -j -U) ;;
  *)      lib="$r_home/lib/libR.so";    nm_args=(-D --defined-only) ;;
esac

[ -f "$lib" ] || { echo "libR not found at $lib" >&2; exit 1; }

# `$NF` is the symbol name on both macOS (-j: name only) and Linux
# (addr type name). Strip the Mach-O leading underscore; keep C identifiers.
nm "${nm_args[@]}" "$lib" \
  | awk 'NF {print $NF}' \
  | sed 's/^_//' \
  | grep -E '^[A-Za-z_][A-Za-z0-9_]*$' \
  | sort -u

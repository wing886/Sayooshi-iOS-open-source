#!/usr/bin/env bash
# Checks that every vendored crate here is exactly its crates.io release plus
# the .patch file next to it, so nothing else can drift in unnoticed.
#
#   platform/switch/patches/verify.sh            # verify
#   platform/switch/patches/verify.sh --upstream # also report whether the
#                                                # newest release still needs it
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "${WORK:?}"' EXIT

# name version sha256-of-.crate patch-file
CRATES=(
  "unwinding 0.2.10 4b134ada16dda9e435abe2a6d76a01d497bc60707357845a15f9b0ed42dc88ce unwinding-0.2.10-catch-unwind-i32.patch"
)

status=0
for entry in "${CRATES[@]}"; do
  read -r name version sum patch <<<"$entry"
  crate="$WORK/$name-$version.crate"
  curl -sSfL -o "$crate" "https://static.crates.io/crates/$name/$name-$version.crate"
  echo "$sum  $crate" | sha256sum -c --quiet -
  tar -xzf "$crate" -C "$WORK"
  (cd "$WORK/$name-$version" && patch -s -p1 <"$HERE/$patch")
  if diff -r -x .cargo_vcs_info.json -x Cargo.toml.orig -x .cargo-ok \
      "$WORK/$name-$version" "$HERE/$name-$version"; then
    echo "ok: $name-$version = crates.io + $patch"
  else
    echo "MISMATCH: $name-$version differs from crates.io + $patch" >&2
    status=1
  fi

  if [[ "${1:-}" == "--upstream" ]]; then
    prefix="$(echo "$name" | cut -c1-2)/$(echo "$name" | cut -c3-4)"
    latest="$(curl -sSf "https://index.crates.io/$prefix/$name" | tail -1 | sed 's/.*"vers":"\([^"]*\)".*/\1/')"
    echo "   newest $name on crates.io: $latest"
    if [[ "$latest" != "$version" ]]; then
      echo "   -> try $latest: if its src/panicking.rs no longer needs $patch, drop the vendored copy."
    fi
  fi
done
exit $status

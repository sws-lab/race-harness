#!/usr/bin/env bash
set -euo pipefail

# Automate downloading/cloning the artifacts listed in README.md.
# Usage: ./fetch_artifacts.sh [target-directory]
# Defaults to the current directory. Override with TARGET_ROOT env var as well.

ROOT="${1:-${TARGET_ROOT:-$PWD}}"

REPOS=(
  "https://github.com/sws-lab/race-harness-generator f044f58be17e368c5e6248baaae5eb4480f3da54 race-harness-generator"
  "https://github.com/sws-lab/race-harness-goblint a4cf2ef14e5da1ce7e5ef29725b16f94437f0425 race-harness-goblint"
  "https://github.com/sws-lab/race-harness-cil aa943ed60749a481db61d3d74e8cd1b21ce80fe3 race-harness-cil"
)

TARBALLS=(
  "https://www.kernel.org/pub/linux/kernel/v6.x/linux-6.14.9.tar.xz 390cdde032719925a08427270197ef55db4e90c09d454e9c3554157292c9f361 linux-6.14.9.tar.xz"
  "https://github.com/utwente-fmt/ltsmin/releases/download/v3.0.2/ltsmin-v3.0.2-linux.tgz 9112846d1b3f6c4db25179a5712ffc25b98c4c26799250875cba859808de07a1 ltsmin-v3.0.2-linux.tgz"
)

command -v git >/dev/null 2>&1 || { echo "git is required" >&2; exit 1; }
command -v curl >/dev/null 2>&1 || { echo "curl is required" >&2; exit 1; }
command -v sha256sum >/dev/null 2>&1 || { echo "sha256sum is required" >&2; exit 1; }

mkdir -p "$ROOT"

ensure_repo() {
  local url="$1"
  local ref="$2"
  local dest="$3"
  local path="$ROOT/$dest"

  if [ -d "$path" ] && [ ! -d "$path/.git" ]; then
    echo "Refusing to overwrite non-git directory $path" >&2
    return 1
  fi

  if [ ! -d "$path/.git" ]; then
    git init "$path"
    git -C "$path" remote add origin "$url"
  fi

  git -C "$path" fetch --depth=1 origin "$ref"
  git -C "$path" checkout --detach "$ref"
  git -C "$path" submodule update --init --recursive --depth=1 || true
}

ensure_tarball() {
  local url="$1"
  local expected_sha="$2"
  local name="$3"
  local dest="$ROOT/$name"
  local tmp="${dest}.partial"

  if [ -f "$dest" ]; then
    if echo "$expected_sha  $dest" | sha256sum -c - >/dev/null 2>&1; then
      echo "$name already present and checksum OK"
      return 0
    fi
    echo "Existing $name has wrong checksum; redownloading" >&2
    rm -f "$dest"
  fi

  curl -L --fail "$url" -o "$tmp"
  echo "$expected_sha  $tmp" | sha256sum -c -
  mv "$tmp" "$dest"
}

echo "Fetching repositories into $ROOT"
for entry in "${REPOS[@]}"; do
  read -r url ref dest <<<"$entry"
  echo "-> $dest ($ref)"
  ensure_repo "$url" "$ref" "$dest"
done

echo "Fetching tarballs into $ROOT"
for entry in "${TARBALLS[@]}"; do
  read -r url sha name <<<"$entry"
  echo "-> $name"
  ensure_tarball "$url" "$sha" "$name"
done

cat <<EOF
Done.
Layout now matches the README expectations:
  $ROOT/
    race-harness (assumed existing; this script does not clone it)
    race-harness-generator
    race-harness-goblint
    race-harness-cil
    linux-6.14.9.tar.xz
    ltsmin-v3.0.2-linux.tgz
EOF

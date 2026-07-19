#!/usr/bin/env bash
# Install or uninstall Ronin binary, .desktop entry, and icons to XDG/Linux paths.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PACKAGING_DIR="${ROOT}/packaging"
DEFAULT_BINARY="${ROOT}/target/release/ronin"

MODE="user"
ACTION="install"
DRY_RUN=0
PREFIX=""
BINARY_SOURCE=""

usage() {
  cat <<'EOF'
Usage: scripts/install.sh [options]

Options:
  --user              Install to ~/.local (default)
  --system            Install to /usr/local (may need root)
  --prefix DIR        Override install prefix
  --binary PATH       Binary to install (default: target/release/ronin)
  --uninstall         Remove installed files for the chosen prefix
  --dry-run           Print planned operations without writing
  -h, --help          Show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --user) MODE="user"; shift ;;
    --system) MODE="system"; shift ;;
    --prefix) PREFIX="$2"; shift 2 ;;
    --binary) BINARY_SOURCE="$2"; shift 2 ;;
    --uninstall) ACTION="uninstall"; shift ;;
    --dry-run) DRY_RUN=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown option: $1" >&2; usage >&2; exit 1 ;;
  esac
done

if [[ -z "$PREFIX" ]]; then
  if [[ "$MODE" == "system" ]]; then
    PREFIX="/usr/local"
  else
    PREFIX="${HOME}/.local"
  fi
fi

BINARY_SOURCE="${BINARY_SOURCE:-$DEFAULT_BINARY}"

BIN_DEST="${PREFIX}/bin/ronin"
DESKTOP_DEST="${PREFIX}/share/applications/ronin.desktop"
ICON_SVG_DEST="${PREFIX}/share/icons/hicolor/scalable/apps/ronin.svg"
declare -A ICON_PNG_DEST=(
  [48]="${PREFIX}/share/icons/hicolor/48x48/apps/ronin.png"
  [128]="${PREFIX}/share/icons/hicolor/128x128/apps/ronin.png"
  [256]="${PREFIX}/share/icons/hicolor/256x256/apps/ronin.png"
)

run() {
  if [[ "$DRY_RUN" -eq 1 ]]; then
    printf '+';
    printf '%q ' "$@"
    printf '\n'
  else
    "$@"
  fi
}

install_file() {
  local src="$1" dest="$2" mode="${3:-0644}"
  run install -D -m "$mode" "$src" "$dest"
}

do_install() {
  if [[ ! -f "$BINARY_SOURCE" ]]; then
    echo "Binary not found: $BINARY_SOURCE" >&2
    echo "Build first: cargo build --release -p ronin" >&2
    exit 1
  fi
  if [[ ! -f "${PACKAGING_DIR}/ronin.desktop" ]]; then
    echo "Missing desktop file: ${PACKAGING_DIR}/ronin.desktop" >&2
    exit 1
  fi

  install_file "$BINARY_SOURCE" "$BIN_DEST" 0755

  # Rewrite Exec= to the installed binary path for reliable launcher starts.
  local desktop_tmp
  desktop_tmp="$(mktemp)"
  sed "s|^Exec=.*|Exec=${BIN_DEST}|" "${PACKAGING_DIR}/ronin.desktop" >"$desktop_tmp"
  install_file "$desktop_tmp" "$DESKTOP_DEST" 0644
  rm -f "$desktop_tmp"

  install_file \
    "${PACKAGING_DIR}/icons/hicolor/scalable/apps/ronin.svg" \
    "$ICON_SVG_DEST" 0644
  for size in 48 128 256; do
    install_file \
      "${PACKAGING_DIR}/icons/hicolor/${size}x${size}/apps/ronin.png" \
      "${ICON_PNG_DEST[$size]}" 0644
  done

  if [[ "$DRY_RUN" -eq 0 ]]; then
    if command -v update-desktop-database >/dev/null 2>&1; then
      update-desktop-database "${PREFIX}/share/applications" 2>/dev/null || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
      gtk-update-icon-cache -f -t "${PREFIX}/share/icons/hicolor" 2>/dev/null || true
    fi
    echo "Installed Ronin to ${PREFIX}"
    echo "  binary:  ${BIN_DEST}"
    echo "  desktop: ${DESKTOP_DEST}"
  fi
}

do_uninstall() {
  local paths=(
    "$BIN_DEST"
    "$DESKTOP_DEST"
    "$ICON_SVG_DEST"
    "${ICON_PNG_DEST[48]}"
    "${ICON_PNG_DEST[128]}"
    "${ICON_PNG_DEST[256]}"
  )
  for path in "${paths[@]}"; do
    if [[ "$DRY_RUN" -eq 1 ]]; then
      printf '+ rm -f %q\n' "$path"
    else
      rm -f "$path"
    fi
  done
  if [[ "$DRY_RUN" -eq 0 ]]; then
    echo "Removed Ronin files from ${PREFIX}"
  fi
}

case "$ACTION" in
  install) do_install ;;
  uninstall) do_uninstall ;;
esac

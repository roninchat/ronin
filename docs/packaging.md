# Linux packaging

Ronin ships a freedesktop `.desktop` entry, hicolor icons, and an install helper for standard Linux paths.

## Assets

| Path | Purpose |
|------|---------|
| `packaging/ronin.desktop` | Launcher entry (`Name`, `Exec`, `Icon`, `Categories`, `Comment`) |
| `packaging/icons/hicolor/*/apps/ronin.png` | Raster icons at 48×48, 128×128, 256×256 |
| `packaging/icons/hicolor/scalable/apps/ronin.svg` | Scalable icon (Catppuccin mauve accent; transparent background for light/dark themes) |

## Install / uninstall

Build a release binary first:

```bash
cargo build --release -p ronin
```

User install (default prefix `~/.local`):

```bash
make install
# or
./scripts/install.sh --user
```

System install (`/usr/local`, may need root):

```bash
make install-system
# or
sudo ./scripts/install.sh --system
```

Dry-run (no writes):

```bash
make install-dry-run
./scripts/install.sh --dry-run --prefix ~/.local
```

Uninstall / clean:

```bash
make uninstall
make clean-install
./scripts/install.sh --uninstall --user
```

Override prefix or binary:

```bash
make install PREFIX=/opt/ronin BINARY=./target/release/ronin
```

After install, the desktop entry’s `Exec=` points at the installed binary. Ensure `~/.local/bin` (or `/usr/local/bin`) is on your `PATH` if you launch from a terminal.

## Global shortcut: `ronin --quick`

Bind a keyboard shortcut so Quick mode opens from anywhere.

### GNOME

1. Open **Settings → Keyboard → View and Customize Shortcuts → Custom Shortcuts**.
2. Add a shortcut:
   - **Name:** Ronin Quick
   - **Command:** `ronin --quick` (or the full path, e.g. `~/.local/bin/ronin --quick`)
   - **Shortcut:** choose a chord that does not conflict (e.g. `Super+Space` if free, or `Ctrl+Alt+R`)

CLI alternative (GNOME 42+):

```bash
# Pick an unused custom keybinding slot, then:
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings \
  "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/ronin-quick/']"

gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/ronin-quick/ name 'Ronin Quick'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/ronin-quick/ command 'ronin --quick'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/ronin-quick/ binding '<Super>r'
```

Adjust the binding string if `Super+R` is already used.

### KDE Plasma

1. Open **System Settings → Shortcuts → Custom Shortcuts** (or **Keyboard → Shortcuts** on newer Plasma).
2. Add a new shortcut / command:
   - **Command / script:** `ronin --quick`
   - **Trigger:** your preferred global chord (e.g. `Meta+R`)

Or with `kwriteconfig6` / Plasma’s custom command UI: create an entry that runs `ronin --quick` and assign a global accelerator under **Global Shortcuts**.

### Sway

Add to your Sway config (`~/.config/sway/config`):

```sway
# Ronin Quick overlay
bindsym $mod+r exec ronin --quick
```

Reload:

```bash
swaymsg reload
```

If `ronin` is not on the login environment `PATH`, use an absolute path:

```sway
bindsym $mod+r exec $HOME/.local/bin/ronin --quick
```

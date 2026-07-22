# Perf Harness uses isolated XDG paths by default

Harness runs must not touch the developer’s normal Ronin config/data (`~/.config/ronin`, `~/.local/share/ronin`). Official scenarios load fixtures into isolated paths so Perf Judgments stay reproducible and user data stays out of the tooling loop.

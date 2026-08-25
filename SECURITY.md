# Security policy

## Reporting a vulnerability

Please report privately through GitHub's **Report a vulnerability** button on
the Security tab, rather than opening a public issue.

Include what an attacker can do and how to reproduce it. A proof of concept
helps, but a clear description of the mechanism is worth more than a payload.

## What is in scope

VDrop takes an address from the user, asks servers about it, and writes files
to disk. Most of the risk lives on those three edges:

- **Path handling.** Titles and filenames come from remote pages and can be
  hostile. Everything that reaches disk passes through
  `vdrop_download::safe_join`, which sanitises the name and verifies the
  result stays inside the target folder. A way past that is a vulnerability.
- **Subprocess arguments.** FFmpeg and yt-dlp are invoked with values derived
  from remote input. Argument injection is in scope.
- **The clipboard watcher.** It reads the clipboard when enabled and
  deliberately **never fetches** what it finds - clipboards carry internal
  addresses, signed links and password resets. Anything that makes it request
  a caught address on its own is a vulnerability.
- **Provider chain.** `kick-video.download` is contacted only for Kick VOD
  addresses and only after every other provider has failed. Anything that
  widens that is in scope.

## What is not in scope

- The remote services themselves. If yt-dlp mis-extracts a site, that belongs
  to yt-dlp; if a CDN serves the wrong file, that belongs to the CDN.
- Missing code signing. The installer is unsigned, so SmartScreen warns on
  first run. That is a known limitation, listed in `docs/DURUM.md`, not a
  vulnerability report.
- Downloading content you are not allowed to download. See the intended-use
  section of the README.

# rsbt description

<img alt="RSBT" src="_rsbt-web-common/web/img/01.svg" width="256px" height="256px">

This is placeholder for future migration of <https://github.com/kilork/rustorrent>.

## Legal

Source code except logo / artwork images is dual-licensed under MIT or the [UNLICENSE](http://unlicense.org/).

<a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons Licence" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/88x31.png" /></a><br />Artwork is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License</a>.

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt = "0.1"
```

## Development information

### Configuration directory

Default config location: `$HOME/.rsbt/`

Default tree structure:

```text
├── download
│   ├── Big\ Buck\ Bunny.en.srt
│   ├── Big\ Buck\ Bunny.mp4
│   └── poster.jpg
├── torrents
│   ├── big-buck-bunny.torrent
│   └── big-buck-bunny.torrent.state
└── torrents.toml
```

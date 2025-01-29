# vt7packer

Unpacks and repacks game files from the Virtual Theatre 7 engine from Revolution Software Limited.

## Installation

Either download the binaries from the Github Releases page or, if you have rust installed, clone
the repo and build it with `cargo build --release`.

## Usage

You can find usage information via the cli:
```
vt7packer help
```

See also [USAGE.md](USAGE.md) for specific examples.

## VT7 Games

Tested:

* Broken Sword: Shadow of the Templars - Reforged

Untested:
* Broken Sword: Serpents Curse

Older VT games that might also work:
* Broken Sword: Shadow of the Templars
* Broken Sword: Smoking Mirror

## Supported file formats

* Translated into useable form
	* ogg (language.osa)
	* osa
	* sav
	* swordtext (text.vt7a)
	* ttf (common.vt7a)
	* txt
	* vt7a
	* webm (movie.vt7a)
	* webp (graphics.vt7a)
	* xml
* Untranslated
	* ChrTxt (sword.vt7a)
	* Compat (sword.vt7a)
	* LyrIdx (sword.vt7a)
	* Script (sword.vt7a)
	* Sprite (sword.vt7a)
* Missing support
	* STR (graphics.vt7a)
	* MEG (graphics.vt7a)
	* GTX (graphics.vt7a)
	* CDT (sword.vt7a)
	* GRID (sword.vt7a)
	* LAYER (sword.vt7a)
	* Various fileformats without magic header (common.vt7a, sword.vt7a)

## Disclaimer

Please also read the LICENSE and keep backups of your original game files.
This software is still alpha and may contain bugs, flaws and errors. Use at
your own risk.

This software is neither supported nor endorsed by Revolution Software Limited.

"Revolution" and "Broken Sword" are trademarks of Revolution Software
Limited.
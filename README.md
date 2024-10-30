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

However the quick start version is that you can extract an archive with the `decode` command:
```
vt7packer decode text.vt7a
# Will create out/#######.vt7a.json and out/#######.vt7a.d/*
```

You can pack files into an archive by sending the archive json (created by `decode`)
to the `encode` command:
```
vt7packer encode #######.vt7a.json
# Will take #######.vt7a.json and #######.vt7a.d/* and pack them into a new #######.vt7a
```

## Example

This is an example of how you could create your own translation mod.

1. Backup your `text.vt7a` from your game folder.
2. Extract the textlines of the game via `vt7packer decode /path/to/text.vt7a`.
3. In the output folder (default `./out/`) you can now find `.sword_text.json` files which correspond to a single language each.
4. Replace all lines in a single `.sword_text.json` with your translations, while keeping the `identifier` and `offset` as it is.
5. Pack the archive back together via `vt7packer encode out/########.vt7a.json`.
6. You should now have a `########.vt7a` in your output folder. Rename it to `text.vt7a` and replace the one in your game folder.
7. Start the game and select the language that you just replaced with your own translations.

You could now also change the flag of the replaced language with your own flag by editing `graphics_2x.vt7a` (and the others) in the same way.

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
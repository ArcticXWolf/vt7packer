# Usage

You can find usage information via the cli:
```
vt7packer help
```

## Extracting files from the game

You can extract an archive with the `decode` command:
```
vt7packer decode text.vt7a
# Will create out/#######.vt7a.json and out/#######.vt7a.d/*
```

## Packing files into an archive

You can pack files into an archive by sending the archive json (created by `decode`)
to the `encode` command:
```
vt7packer encode #######.vt7a.json
# Will take #######.vt7a.json and #######.vt7a.d/* and pack them into a new #######.vt7a
```

## Create your own subtitle translation mod

This is an example of how you could create your own translation mod.

1. Backup your `text.vt7a` from your game folder.
2. Extract the textlines of the game via `vt7packer decode /path/to/text.vt7a`.
3. In the output folder (default `./out/`) you can now find `.sword_text.json` files which correspond to a single language each.
4. Replace all lines in a single `.sword_text.json` with your translations, while keeping the `identifier` and `offset` as it is.
5. Pack the archive back together via `vt7packer encode out/########.vt7a.json`.
6. You should now have a `########.vt7a` in your output folder. Rename it to `text.vt7a` and replace the one in your game folder.
7. Start the game and select the language that you just replaced with your own translations.

You could now also change the flag of the replaced language with your own flag by editing `graphics_2x.vt7a` (and the others) in the same way.

## Edit a savegame

The `decode` and `encode` command can also convert savegames into a `.json` file.
This way you can view the contents of your save and edit them.

1. Backup your savegame file.
2. Extract your savegame into a human-readable file via `vt7packer decode /path/to/BS1R_ManualSave_X.sav`
3. In the output folder (default `./out/`) you can now find a `.sav.json` file.
4. View and edit this file to your liking.
5. Create a valid savegame from the human-readable file via `vt7packer encode out/########.sav.json`
6. You should now have a `########.sav` in your output folder. Rename it to `BS1R_ManualSave_X.vt7a` and place it in the savegame path of your game. (Warning, Steam Cloud Saves may replace or delete files during game startup. In this case place the file while the game is running and then restart it.)
7. Start the game with your new savegame.

## Check savegame for the EVERYBODYS_BUZZING achievement

1. Convert your savegame into a human-readable file like described in `Edit a savegame` above.
2. Search the file for all entries named `BUZZER` and `SHOCK` (should be 13 `BUZZER` and 1 `SHOCK`).
3. If the value behind an entry is `0`, then you did not buzz that person yet, if it is a `1`, then you did buzz them.
4. If you want, then edit all entries into a `1`.
5. Create a valid savefile from your changes like described in `Edit a savegame` above.
6. Play until you can talk to the soccer fan in the train. As soon as you talk with him about the buzzer, you get the achievement.
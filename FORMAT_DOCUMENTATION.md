# Format documentation

This here documents the file formats of VT7. Any contributions are welcome.

## VT7A

This file format is completely little-endian.

```
* HEADER
* List of DIRECTORY_ENTRY
* List of FILE_CONTENT
```

HEADER
```
* uint32 magic bytes ("VT7A")
* uint32 version number (0x00000002 or 0x00000003)
* uint32 (speculation: it is a timestamp of archive creation?)
* uint32 number of DIRECTORY_ENTRY
```

DIRECTORY_ENTRY
```
* uint32 file identifier
* uint32 file offset (from beginning of archive)
* uint32 file size
* uint32 compressed file size (0 if no compression)
```

FILE_CONTENT
```
If compression is used: 
  On archive version 2: zlib compressed bytes of the file
  On archive version 3: zstd compressed bytes of the file
If no compression: raw bytes of the file
```

## OSA

This file format is completely little-endian.

```
* HEADER
* List of DIRECTORY_ENTRY
* List of FILE_CONTENT
```

HEADER
```
* uint32 magic bytes ("AUFS")
* uint32 number of DIRECTORY_ENTRY
```

DIRECTORY_ENTRY
```
* uint32 file identifier
* uint32 file offset (from beginning of archive)
* uint32 file size
```

FILE_CONTENT
```
raw bytes of the file
```

## Language files (from text.vt7a)

This file format is completely little-endian.

```
* HEADER
* List of DIRECTORY_ENTRY
* List of TEXTLINES
```

HEADER
```
* uint32 magic bytes ("TEXT")
* uint32 zeros
* uint32 number of DIRECTORY_ENTRY
```

DIRECTORY_ENTRY
```
* uint32 textline identifier
* uint32 file offset (from beginning of archive)
```

TEXTLINES
```
UTF-8 encoded text lines terminated with \0
```

## Savegame

This file format is completely little-endian.

This file format is an extension of the original savegame format. It add the
reforged data at the end of the file.

```
List of 150x SECTION_OR_ROOM_DATA
List of 1179x GAME_VARIABLE
List of 85x PLAYER_DATA
REFORGED_DATA
```

SECTION_OR_ROOM_DATA
(see SCUMMVM sourcecode for meaning)
```
* uint32 value
```

GAME_VARIABLE
(in the order of [variable names here](src/codecs/save_codec.rs))
```
* uint32 value
```

PLAYER_DATA
(see SCUMMVM sourcecode for meaning)
```
* uint32 value
```

REFORGED_DATA
```
* uint32 fixed one (0x00000001)
* float32 playtime in seconds
* uint64 savegame creation timestamp in 10^-7 seconds
* 11x uint64 zeros
```
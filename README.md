SMT: Devil Summoner Disection Tools

These tools make it easy to split up and convert some of the .bin files included in SMT:Devil Summoner for the PSP.

Included utils:
- gim2png - converts GIM files to PNG; not a universal tool, only really written for this use case.
- binextract - takes a .bin archive file and extracts all the items in the file to seperate files, trying to match headers for filetypes and renaming accordingly. By default checks for the last entry being the string 'PSPCHECK' as per the game logic.
- binsplit - some files are of a slightly different format (usually the *all.bin files), and these contain multiple files as well. Some of extracted are themselves .bin archives that can be further split by the above tool.

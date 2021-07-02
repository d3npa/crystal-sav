# crystal-sav
Save-state (EDGBx7) patching on Pocket Monsters Crystal (JPN)

The Japanese release of Pocket Monsters Crystal uses 64K of SRAM and an MBC30. Many emulators do not quite support these specs, some only going up to 32K of SRAM, and others still not supporting an MBC30. The Everdrive GB x7 also cannot emulate the MBC30, and tries to make do emulating an MBC3 instead, which causes some glitches when saving the game the usual way. 

Fortunately, using savestates appears to work without issue! However this makes patching even harder, as while tech information on the Japanese release is already scarce, I did not find any information whatsoever about the particular `.sav` savestate files I was getting.

This small patcher tool is not a product - it's just an experiment as I play along, patching things I care to. The interface is limited - I edit the code directly when I need something new. It only took around a day to get to this point, and I'm sure someone somewhere could do more if only they cared to (lol)
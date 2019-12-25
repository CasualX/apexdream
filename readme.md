Apex Legends external cheat for UnKnoWnCheaTs in C++
====================================================

Released on [UnKnoWnCheaTs](https://www.unknowncheats.me/forum/apex-legends/368409-external-apexbot.html#post2651040).

How to compile
--------------

**EasyAntiCheat bypass not included!**

See `src/process.cpp` and implement the following routines with your bypass:

* `read_process_memory`
* `write_process_memory`
* `virtual_query_ex`
* `get_mapped_file_name`

Using whichever C++ compiler you fancy point it at `src/stdafx.cpp` or compile the other cpp files individually.

Features
--------

Basic framework for reading structured data from the game's memory.

Change configuration by editing the sources and recompiling.

### Highlight enemies

Found in `highlight.hpp`.

Using the game's own highlight feature enabled for enemies.

### Soft AimAssist

Found in `aimassist.hpp`.

Hold MOUSE4 to gently nudge your aim in the direction of the nearest target.

License
-------

Licensed under [GPL 3.0 License](https://opensource.org/licenses/GPL-3.0), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.

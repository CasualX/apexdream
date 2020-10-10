Apex Legends external cheat in C++
==================================

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

For testing and development without EasyAntiCheat bypass:

1. Locate the folder `C:\Program Files (x86)\Origin Games\Apex`.
2. Rename `EasyAntiCheat_launcher.exe` to something else, eg. `EasyAntiCheat_launcher.exe.bak`.
3. Rename `r5apex.exe` to `EasyAntiCheat_launcher.exe` and launch it.
4. To restore functionality simply undo the previous steps.

Some modifications to the source code are also needed:

1. Checkout the `win32` branch of this repo with `git checkout win32`.
2. In `process.hpp` find the defition of `PROCESS_NAME` and set it to `L"EasyAntiCheat_launcher.exe"`.

This launches the game without EasyAntiCheat enabled.
You can now load the Firing Range and mess around.
If you join an online game you will be kicked shortly after landing on the ground.

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

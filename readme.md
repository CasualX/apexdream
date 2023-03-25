The Apex Legends Dream
======================

**There is no bypass for EasyAntiCheat! You have to implement your own EasyAntiCheat bypass.**

This is an external cheat for Apex Legends written in Rust. The cheat is embeddable, you implement the API to access Apex Legends and in return you get a bunch of features.

See [here](examples/apexdream-win32.rs) for an example of how to use the API.

Features
--------

### Aim assist

Advanced legitimate aim assist with many customizable settings. Its focus is on being as legitimate as possible, while still being effective. It works by calculating mouse movement commands, it does not write viewangles.

It uses advanced trajectory calculations based on sampled data for accurate prediction.

Some of its features:

* Aim at visible enemies only.
* Aim automatically when firing.
* Customizable _hold to aim_ key.
* Distance and zoom based FOV scaling.
* Configure vertical and horizontal aim strength independently.
* Smooth aim powered by a [PID controller](https://en.wikipedia.org/wiki/PID_controller).
* Aim ramps up and fades out smoothly.
* Aim strength drops temporarily when the target is moving erratically.
* Aim spine system so *you* are in control which body part to aim at.
* Aim priority system to prioritize players but still aim at other things like knocked players or abilities.

### Triggerbot

When locked onto an enemy with the aimbot, can trigger when the enemy is in the crosshair. Works with close range guns like shotguns and long range guns like snipers where it uses prediction to trigger at the right time.

Uses humanized clicking, customized per weapon.

### Recoil Control System

Simple humanized RCS that removes most of the weapon recoil only (does not remove weapon sway).

Configure vertical and horizontal compensation independently.

### ESP

Customizable ESP for many objects in the game:

* Draw 2D bounding boxes, with separate mode for players behind cover.
* Team based coloring so you know who has the advantage in a 3rd party.
* Draw player names.
* Draw health and armor bars.
* Draw loot icons (only for items that are useful or upgrades).
* Draw skeleton.
* Draw fading trail as the player moves around.
* Draw where to aim to hit the head with prediction.
* Draw where to aim frag grenades in the sky to hit them from above.
* ...

### Highlight

Ye olde highlighter. Highlights players, loot, and other objects using ingame effects.

### Observers

Draws a list of dead players (when their team is dead) and whether or not they're spectating you.

### Radar

Simple 2D radar that shows live enemy players as dots around your crosshair.

### Ring damage indicator

When near or outside the ring, a damage indicator shows when you will take the next tick of damage. Additionally a timer counts down to the last possible time for you to start healing (phoenix, medkit, syringe) before you won't have enough time to heal before dying to the ring.

### Scripts

A bunch of little automations:

* Legitimized rapidfire and bunnyhop.
* Auto reload, auto tac reload and animation cancel reloads.
* Auto tap strafe.
* Thirdperson and freecam toggle.
* Simple humanized fast loot and auto loot.

### Debugger

A simple debugger to visualize the game state live.

Update offsets
--------------

The offsets are provided statically by `gamedata.ini` and parsed.

Manually update the offsets by pasting the INI output from the [Apex Legends Offset Dumper](https://casualhacks.net/apexdream/apexdumper.html) in `offsets/stdout.ini`. Then run `cargo run --example gamedata >gamedata.ini` to update the gamedata offsets.

The cheat can dump the game for you, but it requires a few extra steps. See if you can figure it out 😉. Otherwise lurk around on [UnKnoWnCheaTs](https://www.unknowncheats.me/forum/apex-legends/319804-apex-legends-reversal-structs-offsets.html) and wait for people to post updated offsets.

## License

Licensed under [GPL 3.0 License](https://opensource.org/licenses/GPL-3.0), see [license.md](license.md).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.

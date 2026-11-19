`systemi` is an attempt to write portable versions of useful, desktop daemons that are under `systemd`'s purview, such as `timedate1` or `networkmanager`. Most of these services expose a D-Bus interface with a backend for the platform. The daemons are often tied to Linux and especially Linux + `systemd`, mostly for historical reasons and lack of inertia for ports.

To squelch any debates, I am not against systemd. My Linux installations _all_ use systemd. I am primarily implementing this for FreeBSD, but I know that Linux folk would appreciate it too.

## Why don't you work with upstream?

Some of the daemons are gorilla glued to Linux, especially `networkmanager`. Adding FreeBSD support would involve hacking the source code up then implementing the requisite backends. That's a huge commitment. In contrast, if I fail at this I can archive the repository then go play video games instead.

## Name

Pronounced as "sys-tem-e" as one word or "system-e" - whichever floats your socks and rocks your boat.

## License
The code is mostly licensed under `BSD-2-Clause` Some of the code is licensed under GPL because I either based it off of GPL code or copied my previous implementations that are licensed under the GPL. Every file uses SPDX licenses, so you can grep if interested.

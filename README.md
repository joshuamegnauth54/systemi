`timedate_bsd` is a portable implementation of [org.freedesktop.timedate1](https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.timedate1.html) targeting the BSDs, specifically FreeBSD. My goal to develop small daemons that implement interfaces used by desktop environments so that users of the BSDs and non-systemd Linux have a comparable experience with systemd Linux.

To squelch any debates, I am not against systemd. My Linux installations _all_ use systemd. I am primarily implementing this for FreeBSD, but I know that Linux folk would appreciate it too.

## License
The code is mostly licensed under `BSD-2-Clause` Some of the code is licensed under GPL because I either based it off of GPL code or copied my previous implementations that are licensed under the GPL. Every file uses SPDX licenses, so you can grep if interested.

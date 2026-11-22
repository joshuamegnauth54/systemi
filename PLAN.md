## General
For testing, certain libc or syscalls can just be mocked, like sethostname. Vast majority of service can stay the same because paths et cetera aren't hard coded. Tests probably don't need root.

Testing daemon itself can bind to session bus with randomized name.

Handle sigchld plz.

## timedate1

SetTimezone: Service stores a dirfd to zoneinfo. Timezone string is bounded to 48-ish. Directory is walked with resolve beneath to verify timezone. Maybe read the file to determine it's actually a timezone thing.

You may contribute on either Codeberg or GitHub. I'll merge pull requests from either though you should consider Codeberg to be the canonical repository.

If you contribute on GitHub, I'll push your commits upstream then close the PR on GitHub. You will still be credited and your commits will show up on GitHub exactly as you've written them. See FreeBSD's GitHub repositories for examples of this workflow. The reason for this weirdness is that it's easier to have one repository to push to rather than trying to sync Codeberg <-> GitHub bidirectionally.

## Style

I prefer a minimalist, conscious style of Rust for systemi that avoids allocations where possible and uses operating system specific mechanisms. In other words, prefer modern syscalls via Rustix and small abstractions over trying to heavily abstract system differences. These daemons will naturally have a lot of platform specific code. Accept it. Dream of it.

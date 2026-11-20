You may contribute on either Codeberg or GitHub. I'll merge pull requests from either though you should consider Codeberg to be the canonical repository.

If you contribute on GitHub, I'll push your commits upstream then close the PR on GitHub. You will still be credited and your commits will show up on GitHub exactly as you've written them. See FreeBSD's GitHub repositories for examples of this workflow. The reason for this weirdness is that it's easier to have one repository to push to rather than trying to sync Codeberg <-> GitHub bidirectionally.

## Style

I prefer a minimalist, conscious style of Rust for systemi that avoids allocations where possible and uses operating system specific mechanisms. In other words, prefer modern syscalls via Rustix and small abstractions over trying to heavily abstract system differences. These daemons will naturally have a lot of platform specific code. Accept it. Dream of it.

Programmer time is valuable, but not in the way you think. The term implies compromises, implies using a leviathan framework where it's not needed. Our time is valuable in the sense that we should care about craft. If you're writing code, you should care about its quality and the delight of those who will use your code. Think of cooking where you could microwave a frozen pizza to satiate yourself or your comrades, or you could bake a pizza from scratch to cater directly with little compromise.

No compromises. These daemons should be as small as possible with little or no heap allocations and be bounded and secure. I am not an amazing or even a decent programmer, but that is the direction I wish to take and I hope we could achieve it together.

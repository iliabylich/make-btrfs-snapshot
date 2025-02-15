# make-btrfs-snapshot

This package combines two things:

1. Grub hook to additionally generate entries for btrfs snapshot (taken as is from https://github.com/Antynea/grub-btrfs/tree/master)
2. An apt hook that creates a snapshot, removes stale snapshots (by keeping only last 5 items) and updates grub.

# Dependencies

1. `update-grub`
2. `timeshift`

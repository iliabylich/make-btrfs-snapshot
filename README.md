# make-btrfs-snapshot

This package combines two things:

1. Grub hook to generate an entry for each btrfs snapshot
2. An apt hook that creates a snapshot, removes stale snapshots (by keeping only last 5 items) and updates grub.

# Dependencies

1. `update-grub` (from `grub2-common`)
2. `grub-probe` (from `grub-common`)
3. `btrfs` (from `btrfs-progs`)

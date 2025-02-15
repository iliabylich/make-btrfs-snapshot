build:
	cargo build --release

install:
	@install -Dm644 -t "$(DESTDIR)/etc/apt/apt.conf.d/" 80-make-btrfs-snapshot-and-update-grub
	@install -Dm644 -t "$(DESTDIR)/etc/grub.d/" 41_snapshots-btrfs
	@install -Dm755 -t "$(DESTDIR)/usr/bin/" target/release/make-btrfs-snapshot

sync-grub-btrfs:
	wget https://raw.githubusercontent.com/Antynea/grub-btrfs/refs/heads/master/41_snapshots-btrfs

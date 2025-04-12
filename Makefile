build:
	cargo build --release

install:
	@install -Dm0755 target/release/make-snapshot "$(DESTDIR)/usr/bin/make-btrfs-snapshot"
	@install -Dm0755 target/release/generate-grub "$(DESTDIR)/etc/grub.d/42_generate_btrfs_snapshots"
	@install -Dm644 -t "$(DESTDIR)/etc/apt/apt.conf.d/" 80-make-btrfs-snapshot

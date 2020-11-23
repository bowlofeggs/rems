# Where the project is mounted inside the container
podman_mount_dir = /project
# What tag to apply to the built container
podman_tag = rems:dev
# podman flags to mount the code into the container
podman_volume = -v $(CURDIR)/..:$(podman_mount_dir):z
# podman flags to run the container
podman_run = podman run --network=host --rm -it $(podman_volume) -w $(podman_mount_dir) $(podman_tag)

build:
	podman build --pull -t $(podman_tag) $(podman_volume) --force-rm=true .

shell: build
	$(podman_run) bash

test: build
	$(podman_run) cargo test
	$(podman_run) cargo clippy --all-targets --all-features -- -D warnings
	$(podman_run) cargo doc
	$(podman_run) cargo fmt -- --check -v
	$(podman_run) cargo audit

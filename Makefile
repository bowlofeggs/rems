.PHONY: help check audit build clean clippy doc fmt test

# Where the project is mounted inside the container
podman_mount_dir = /rems
# What tag to apply to the built container
podman_tag = rems:dev
# podman flags to mount the code into the container
podman_volume = -v $(CURDIR):$(podman_mount_dir):z -v $(CURDIR)/.cache:/root/.cargo/registry:z
# podman flags to run the container
podman_run = podman run --network=host --rm -it $(podman_volume) -w $(podman_mount_dir) $(podman_tag)

help:  ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

check: fmt audit build clippy test doc  ## Run the full set of CI checks

audit: container  ## Run cargo audit
	# Neither of these has a fix available, so we will ignore them for now.
	$(podman_run) cargo audit --ignore RUSTSEC-2020-0071 --ignore RUSTSEC-2020-0159

build: container  ## Run cargo build
	$(podman_run) cargo build

clean:  ## Clean build artifacts
	$(podman_run) cargo clean || rm -rf target
	podman rmi $(podman_tag) || true
	rm -rf $(CURDIR)/.cache

clippy: container  ## Run cargo clippy
	$(podman_run) cargo clippy --all-targets --all-features -- -D warnings

container:  ## Build the container
	mkdir -p $(CURDIR)/.cache
	podman build --pull -t $(podman_tag) $(podman_volume) --force-rm=true $(CURDIR)

doc: container  ## Run cargo doc
	$(podman_run) cargo doc --no-deps

fmt: container  ## Run cargo fmt
	$(podman_run) cargo fmt -- --check -v

license: container  ## Run cargo-license
	$(podman_run) cargo license

shell: container  ## Run bash inside the development container
	$(podman_run) bash

test: container  ## Run cargo test
	$(podman_run) cargo test

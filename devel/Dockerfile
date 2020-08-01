FROM registry.fedoraproject.org/fedora:31
LABEL maintainer="Randy Barlow <bowlofeggs@fedoraproject.org>"

RUN dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-31.noarch.rpm
RUN dnf install -y ffmpeg gcc python3-bson python3-devel python3-matplotlib

# We need Rust nightly
RUN curl -s https://sh.rustup.rs -sSf | sh -s -- -y
RUN source /root/.cargo/env
ENV PATH /root/.cargo/bin:$PATH
RUN rustup install nightly
RUN cd /devel && rustup override set nightly

# This way we can cache all the build dependencies
RUN cd /devel && cargo build
# This way we can cache all the development dependencies
RUN cd /devel && cargo test
# This way we can cache all the doc dependencies
RUN cd /devel && cargo doc
# This is needed for cargo-audit
RUN dnf install -y openssl-devel
RUN cargo install cargo-audit
# This is useful for finding all the licenses of the bundled libraries
RUN cargo install cargo-license

CMD ["bash"]

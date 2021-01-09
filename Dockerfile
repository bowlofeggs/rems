FROM registry.fedoraproject.org/fedora:33
LABEL maintainer="Randy Barlow <randy@electronsweatshop.com>"

RUN dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-33.noarch.rpm
RUN dnf upgrade -y
RUN dnf install -y ffmpeg gcc python3-bson python3-devel python3-matplotlib

# We need Rust nightly
RUN curl -s https://sh.rustup.rs -sSf | sh -s -- -y
RUN source /root/.cargo/env
ENV PATH /root/.cargo/bin:$PATH
RUN rustup install nightly
RUN cd /rems && rustup override set nightly

# This is needed for cargo-audit
RUN dnf install -y openssl-devel
RUN cargo install cargo-audit
# This is useful for finding all the licenses of the bundled libraries
RUN cargo install cargo-license

CMD ["bash"]

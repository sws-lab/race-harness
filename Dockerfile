# syntax=docker/dockerfile:1.6
FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Core build dependencies for Linux, Goblint/CIL, and helper scripts.
RUN --mount=type=cache,target=/var/cache/apt \
    apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    clang-18 clang-format-18 llvm-18 lld-18 libc6-dev libclang-18-dev \
    cmake ninja-build pkg-config \
    python3 python3-venv python3-pip python3-dev \
    git curl wget ca-certificates jq file rsync unzip xz-utils sudo \
    gawk flex bison libssl-dev libelf-dev dwarves bc kmod cpio libncurses-dev \
    libcap-dev libiberty-dev zlib1g-dev libpopt-dev \
    libgmp-dev libmpfr-dev autoconf gcc-multilib libc6-dev-i386 lib32gcc-13-dev chrpath \
    opam m4 bubblewrap ruby \
    && update-alternatives --install /usr/bin/clang clang /usr/bin/clang-18 180 \
    && update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-18 180 \
    && update-alternatives --install /usr/bin/llvm-ar llvm-ar /usr/bin/llvm-ar-18 180 \
    && update-alternatives --install /usr/bin/llvm-ranlib llvm-ranlib /usr/bin/llvm-ranlib-18 180 \
    && update-alternatives --install /usr/bin/ld.lld ld.lld /usr/bin/ld.lld-18 180 \
    && rm -rf /var/lib/apt/lists/*

# Install uv (used by race-harness-generator setup).
ENV UV_INSTALL_DIR=/usr/local/bin
RUN curl -LsSf https://astral.sh/uv/install.sh | sh

# Enable passwordless sudo for the default Ubuntu user.
RUN echo "ubuntu ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/ubuntu \
    && chmod 0440 /etc/sudoers.d/ubuntu

ENV OPAMYES=1 \
    OPAMCOLOR=never \
    OPAMROOT=/home/ubuntu/.opam \
    WORKDIR=/opt/race

RUN mkdir -p "${WORKDIR}" \
    && chown -R ubuntu:ubuntu "${WORKDIR}"

WORKDIR "${WORKDIR}"
COPY --chown=ubuntu:ubuntu . "${WORKDIR}/race-harness"

USER ubuntu
ENV PATH="/usr/local/bin:${PATH}"

ARG JOBS=8
ENV JOBS=${JOBS} \
    KERNEL_VERSION=6.14.9 \
    LTSMIN_VERSION=3.0.2

# Fetch required repositories and tarballs.
RUN "${WORKDIR}/race-harness/fetch_artifacts.sh" "${WORKDIR}"

# Unpack Linux kernel and LTSmin.
RUN cd "${WORKDIR}" \
    && tar xvf "linux-${KERNEL_VERSION}.tar.xz" \
    && tar xvf "ltsmin-v${LTSMIN_VERSION}-linux.tgz"

# Core environment paths used by builds.
ENV LTSMIN_DIR=${WORKDIR}/v${LTSMIN_VERSION} \
    RACE_HARNESS_DIR=${WORKDIR}/race-harness-generator

# Build race-harness-generator and prime its Python environment.
RUN cd "${WORKDIR}/race-harness-generator" \
    && uv sync --frozen \
    && make -j"${JOBS}"

# Build Goblint and pin CIL.
RUN cd "${WORKDIR}/race-harness-goblint" \
    && make setup \
    && make dev \
    && cd "${WORKDIR}/race-harness-cil" \
    && eval "$(opam env --switch=${WORKDIR}/race-harness-goblint --set-switch)" \
    && opam pin goblint-cil . \
    && cd "${WORKDIR}/race-harness-goblint" \
    && eval "$(opam env --switch=${WORKDIR}/race-harness-goblint --set-switch)" \
    && make

# Build Linux kernel with Clang 18.
RUN cd "${WORKDIR}/linux-${KERNEL_VERSION}" \
    && make allmodconfig LLVM=-18 \
    && make LLVM=-18 -j"${JOBS}"

# Build Linux kernel compilation database.
RUN cd "${WORKDIR}" \
    && ./race-harness/compile_db/extract_compilation_database.py --build-dir "linux-${KERNEL_VERSION}" --db "linux-${KERNEL_VERSION}.db"

# Persist opam environment for the Goblint switch.
RUN eval "$(opam env --switch=${WORKDIR}/race-harness-goblint --set-switch)" \
    && opam env --switch=${WORKDIR}/race-harness-goblint --set-switch > /tmp/opam.sh \
    && sudo install -m 0644 /tmp/opam.sh /etc/profile.d/opam.sh

ENV BASH_ENV=/etc/profile.d/opam.sh \
    PATH=${WORKDIR}/race-harness-goblint/_opam/bin:${WORKDIR}/race-harness-goblint:${PATH}

RUN ln -sf "${WORKDIR}/race-harness/reproduction/eval.sh" "${WORKDIR}/eval.sh"

CMD ["bash", "-lc", "source /etc/profile.d/opam.sh || true; cd /opt/race; exec bash"]

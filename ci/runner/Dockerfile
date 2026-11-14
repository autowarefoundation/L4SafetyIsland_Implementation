# cspell:ignore colcon cuda DBUILD DCMAKE installdependencies libz networkx noninteractive nopasswd passwordless pipefail pkgxml procps psmisc rclcpp rosdep rosdistro runasroot rustup setuptools startswith tlsv yamale zstd
# L4 Safety Island integration-test images.
#
# One Dockerfile, three stages; two of them are images you use:
#
#   swb-build  (throw-away) Autoware's devel image; builds scenario-world-bridge (swb)
#              from source against Autoware into /opt/swb-ws/install.
#   stack      Autoware's runtime image + swb's install + play_launch + the CARLA Python
#              client + the Rust/nano-ros toolchain. Everything an integration run
#              needs, and nothing that talks to GitHub. Run ci/scripts/* in it locally.
#   runner     stack + the GitHub Actions runner and its entrypoint. Only this one
#              ever holds a runner registration.
#
# CARLA is not in either image: it runs as its own container (ci/runner/compose.yaml).
#
# Build (from the repository root; the context is ci/runner only):
#   docker build -f ci/runner/Dockerfile --target stack  -t l4si-ci-stack:dev  ci/runner
#   docker build -f ci/runner/Dockerfile --target runner -t l4si-ci-runner:dev ci/runner
# or: ci/runner/build.sh

ARG ROS_DISTRO=humble
ARG AUTOWARE_VERSION=2.0.0
ARG AUTOWARE_IMAGE=ghcr.io/autowarefoundation/autoware

# ---------------------------------------------------------------------------
# swb-build: compile swb in Autoware's devel image (headers, -dev packages).
# ---------------------------------------------------------------------------
FROM ${AUTOWARE_IMAGE}:universe-devel-${ROS_DISTRO}-${AUTOWARE_VERSION} AS swb-build
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
USER root
ARG ROS_DISTRO
# swb: a commit of https://github.com/NEWSLabNTU/scenario-world-bridge (main).
ARG SWB_REPO=https://github.com/NEWSLabNTU/scenario-world-bridge.git
ARG SWB_REF=031d46294efe21e9e989be9d5c32b90d5faf61dd

ENV RUSTUP_HOME=/usr/local/rustup CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --no-modify-path

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        libz3-dev python3-pip "ros-${ROS_DISTRO}-rclcpp-components" "ros-${ROS_DISTRO}-class-loader" \
        "ros-${ROS_DISTRO}-example-interfaces" "ros-${ROS_DISTRO}-test-msgs" \
    && rm -rf /var/lib/apt/lists/* \
    && pip3 install --no-cache-dir colcon-cargo-ros2

# The checkout, without the CUDA NDT submodule (built separately, not used here).
RUN git clone --filter=blob:none "${SWB_REPO}" /opt/swb-ws \
    && cd /opt/swb-ws \
    && git checkout --quiet "${SWB_REF}" \
    && git -c submodule.third_party/cuda_ndt_matcher.update=none \
        submodule update --init --recursive --jobs 8 \
    && git rev-parse HEAD > /opt/swb-ws/.swb_ref

# The workspace keeps one path in both stages: colcon-cargo-ros2 resolves the generated
# message crates relative to it, and the install is copied to the same path.
WORKDIR /opt/swb-ws
# rosdep over the packages colcon builds (not --from-paths src: the components' deps/
# copies hold templates rosdep refuses). Skipped keys: tier4_localization_launch, which
# swb's Autoware profile names after Autoware 1.5.0 and 2.0.0 renamed
# (autoware_localization_launch; see docs/testing/integration.md, Known gaps), and
# python-yamale-pip, a pip key installed by pip here.
ARG ROSDEP_SKIP_KEYS="tier4_localization_launch python-yamale-pip"
RUN pip3 install --no-cache-dir yamale
RUN source "/opt/ros/${ROS_DISTRO}/setup.bash" \
    && source /opt/autoware/setup.bash \
    && apt-get update \
    && (rosdep update --rosdistro "${ROS_DISTRO}" || true) \
    && DEBIAN_FRONTEND=noninteractive rosdep install -y --rosdistro "${ROS_DISTRO}" \
        --from-paths $(colcon list --base-paths src -p) --ignore-src \
        --skip-keys "${ROSDEP_SKIP_KEYS}" \
    && rm -rf /var/lib/apt/lists/*

# build/ (CMake trees, cargo targets) is a BuildKit cache: a rebuild after a swb bump is
# incremental. Only install/ lands in the image.
RUN --mount=type=cache,target=/opt/swb-ws/build,id=l4si-swb-build \
    source "/opt/ros/${ROS_DISTRO}/setup.bash" \
    && source /opt/autoware/setup.bash \
    && unset CARLA_DIR \
    && export CMAKE_POLICY_VERSION_MINIMUM=3.5 \
    && colcon build \
        --base-paths src --continue-on-error \
        --event-handlers console_direct- console_cohesion- console_stderr+ status- \
        --cmake-args -DCMAKE_BUILD_TYPE=Release -DBUILD_TESTING=OFF \
        --cargo-args --release \
       || { for f in log/latest_build/*/stdout_stderr.log; do [ -s "$f" ] || continue; \
              grep -q '^error' "$f" || continue; echo "=== $f"; grep -E -A30 '^error' "$f" | tr -s ' ' | fold -w 300 | grep -vE '^ *"?-' | tail -c 6000; done; exit 1; } \
    ; cp .swb_ref install/SWB_REF \
    && mkdir -p /opt/swb-pkgxml \
    && for p in $(colcon list --base-paths src -p); do \
         mkdir -p "/opt/swb-pkgxml/$p" && cp "$p/package.xml" "/opt/swb-pkgxml/$p/"; \
       done \
    && find install -name '*.so' -exec strip --strip-unneeded {} + \
    && find install -type f -perm -u+x -path '*/lib/*' -exec sh -c 'file -b "$1" | grep -q ELF && strip --strip-unneeded "$1"' _ {} \; \
    && test -f install/setup.bash

# ---------------------------------------------------------------------------
# stack: what a run needs. No GitHub runner, no credentials.
# ---------------------------------------------------------------------------
FROM ${AUTOWARE_IMAGE}:universe-${ROS_DISTRO}-${AUTOWARE_VERSION} AS stack
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
USER root
ARG ROS_DISTRO
ARG AUTOWARE_VERSION
ARG PLAY_LAUNCH_VERSION=0.34.0
ARG CARLA_VERSION=0.9.16
# The nros CLI release (https://github.com/NEWSLabNTU/nano-ros/releases). A job pins the
# exact toolchain its checkout declares (nros-toolchain.toml) at run time.
ARG NROS_VERSION=0.6.0-nros8
ARG JUST_VERSION=1.59.0
# The repository's rust-toolchain.toml channel, baked in so a job downloads nothing.
ARG RUST_TOOLCHAIN=1.99.0

LABEL org.opencontainers.image.title="l4si-ci-stack" \
      org.opencontainers.image.description="Autoware ${AUTOWARE_VERSION} + scenario-world-bridge + nano-ros toolchain for L4 Safety Island integration tests" \
      org.opencontainers.image.source="https://github.com/autowarefoundation/L4SafetyIsland_Implementation"

# swb's runtime dependencies: rosdep exec keys of the packages it built.
COPY --from=swb-build /opt/swb-pkgxml /tmp/swb-pkgxml
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        python3-pip zstd xz-utils file jq procps iproute2 psmisc build-essential pkg-config \
        "ros-${ROS_DISTRO}-example-interfaces" "ros-${ROS_DISTRO}-test-msgs" \
    && source "/opt/ros/${ROS_DISTRO}/setup.bash" \
    && source /opt/autoware/setup.bash \
    && (rosdep update --rosdistro "${ROS_DISTRO}" || true) \
    && DEBIAN_FRONTEND=noninteractive rosdep install -y --rosdistro "${ROS_DISTRO}" \
        --from-paths /tmp/swb-pkgxml --ignore-src --dependency-types=exec \
        --skip-keys "tier4_localization_launch python-yamale-pip $(ls /opt/autoware | tr '\n' ' ')" \
    && rm -rf /var/lib/apt/lists/* /tmp/swb-pkgxml

# Python tools: play_launch (the launcher swb drives), the CARLA client (pose helper,
# BasicAgent pilots). The system setuptools must stay first (colcon, swb's check).
RUN pip3 install --no-cache-dir \
        "play_launch==${PLAY_LAUNCH_VERSION}" "carla==${CARLA_VERSION}" networkx shapely yamale \
    && python3 -c 'import setuptools,sys; sys.exit(0 if setuptools.__file__.startswith("/usr/lib/python3/") else 1)' \
    && test "$(pip3 show play_launch | sed -n "s/^Version: //p")" = "${PLAY_LAUNCH_VERSION}"

COPY --from=swb-build /opt/swb-ws/install /opt/swb-ws/install

# Rust (for the island's host build) + just + the nros CLI, all under /opt, on PATH for
# every user. Writable by the aw user so jobs can add a component or target.
ENV RUSTUP_HOME=/opt/rust/rustup CARGO_HOME=/opt/rust/cargo NROS_HOME=/opt/nros
ENV PATH=/opt/nros/bin:/opt/rust/cargo/bin:$PATH
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --no-modify-path \
        --default-toolchain "${RUST_TOOLCHAIN}" -c rustfmt,clippy -t armv8r-none-eabihf \
    && curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh \
        | bash -s -- --tag "${JUST_VERSION}" --to /usr/local/bin \
    && curl -fsSL https://raw.githubusercontent.com/NEWSLabNTU/nano-ros/main/scripts/install.sh \
        | NROS_INSTALL_VERSION="${NROS_VERSION}" sh \
    && nros --version \
    && chown -R aw:aw /opt/rust /opt/nros

# swb's host settings: Autoware at /opt/autoware, CARLA served by its own container
# (never started by the stack), logs and caches under /var/lib/l4si (volumes in compose).
COPY host.yaml /etc/swb/host.yaml
COPY env.sh /etc/l4si/env.sh
# ~/autoware_data: the writable data_path swb's Autoware profile insists on.
RUN mkdir -p /var/lib/l4si/logs /var/lib/l4si/cache /home/aw/autoware_data \
    && chown -R aw:aw /var/lib/l4si /home/aw/autoware_data \
    # HOST_UID remaps aw at start (Autoware's entrypoint) without a chown: keep the
    # shared directories writable for whichever uid aw ends up with.
    && chmod 1777 /var/lib/l4si/logs /var/lib/l4si/cache /home/aw/autoware_data \
    && printf '%s\n' "AUTOWARE_VERSION=${AUTOWARE_VERSION}" "SWB_REF=$(cat /opt/swb-ws/install/SWB_REF)" \
        "PLAY_LAUNCH_VERSION=${PLAY_LAUNCH_VERSION}" "CARLA_VERSION=${CARLA_VERSION}" \
        "NROS_VERSION=${NROS_VERSION}" "RUST_TOOLCHAIN=${RUST_TOOLCHAIN}" > /etc/l4si/versions

# Autoware's entrypoint stays: it sources ROS and Autoware and drops to the aw user.
WORKDIR /home/aw
CMD ["/bin/bash"]

# ---------------------------------------------------------------------------
# runner: stack + the GitHub Actions runner. Ephemeral: one job per container.
# ---------------------------------------------------------------------------
FROM stack AS runner
ARG RUNNER_VERSION=2.338.2
ARG RUNNER_SHA256=baebf204307bac6193480852732f0aba2ae97b6c5597c244f4d5558390147c40

LABEL org.opencontainers.image.title="l4si-ci-runner"

RUN mkdir -p /opt/actions-runner && cd /opt/actions-runner \
    && curl -fsSL -o runner.tgz \
        "https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-x64-${RUNNER_VERSION}.tar.gz" \
    && if [ -n "${RUNNER_SHA256}" ]; then echo "${RUNNER_SHA256}  runner.tgz" | sha256sum -c -; fi \
    && tar xzf runner.tgz && rm runner.tgz \
    && ./bin/installdependencies.sh \
    && rm -rf /var/lib/apt/lists/* \
    && chown -R aw:aw /opt/actions-runner \
    # A job has no business with root: drop the image's passwordless sudo.
    && rm -f /etc/sudoers.d/90-user-nopasswd

COPY --chmod=755 entrypoint.sh /usr/local/bin/l4si-runner
ENV RUNNER_ALLOW_RUNASROOT=0
CMD ["/usr/local/bin/l4si-runner"]

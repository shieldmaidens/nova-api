#! /bin/sh

cat << EOF > "$HOME"/.cargo/config.toml
[registries.gitea]
index = "sparse+https://git.r3t.io/api/packages/nova-engine/cargo/"
EOF

cd rust || exit

cargo login --registry gitea "${CARGO_REPOSITORIES_GITEA_TOKEN}"
cargo publish --registry gitea

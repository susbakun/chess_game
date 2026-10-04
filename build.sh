#!/bin/bash
set -e

# Install Rust targets if needed
rustup target add wasm32-unknown-unknown

# Build the client crate specifically — the workspace also contains
# server and shared, neither of which should (or can) target wasm32.
cargo build --release --target wasm32-unknown-unknown -p client

# Create web directory if it doesn't exist
mkdir -p web/out
mkdir -p web/assets

# Bind WebAssembly — output artifact is named after the crate,
# which is now "client", not "chess_game". Workspace builds still
# share one target/ dir at the workspace root, so the path prefix is unchanged.
wasm-bindgen --out-dir ./web/out --target web ./target/wasm32-unknown-unknown/release/client.wasm

# Copy assets — now living under client/assets, not the old root assets/
cp -r client/assets/* web/assets/ 2>/dev/null || true

# Copy index.html
cat > web/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Chess Game</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            display: flex;
            justify-content: center;
            align-items: center;
        }
        canvas { display: block; }
    </style>
</head>
<body>
    <script type="module">
        import init from './out/client.js';
        init();
    </script>
</body>
</html>
EOF

echo "Build complete! Output in ./web/"

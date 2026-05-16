#!/bin/bash
set -e

# Install Rust targets if needed
rustup target add wasm32-unknown-unknown

# Build Rust to WebAssembly
cargo build --release --target wasm32-unknown-unknown

# Create web directory if it doesn't exist
mkdir -p web/out
mkdir -p web/assets

# Bind WebAssembly
wasm-bindgen --out-dir ./web/out --target web ./target/wasm32-unknown-unknown/release/chess_game.wasm

# Copy assets
cp -r assets/* web/assets/ 2>/dev/null || true

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
        import init from './out/chess_game.js';
        init();
    </script>
</body>
</html>
EOF

echo "Build complete! Output in ./web/"

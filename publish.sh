#!/bin/bash
set -e  # Exit on any error

# Function to check if we should continue
confirm() {
  read -p "$1 (y/n): " choice
  case "$choice" in 
    y|Y ) return 0;;
    * ) return 1;;
  esac
}

# Step 1: Log in to crates.io if needed
echo "===== Step 1: Logging in to crates.io ====="
if ! confirm "Do you already have a crates.io token set up?"; then
  echo "Please get a token from https://crates.io/me"
  cargo login
fi

# Step 2: Verify packages and do dry runs
echo "===== Step 2: Verifying packages ====="

echo "Checking roblox-rs-core..."
cd roblox-rs-core
cargo package --list
cargo publish --dry-run
cd ..

echo "Checking roblox-rs-ecs..."
cd roblox-rs-ecs
cargo package --list
cargo publish --dry-run
cd ..

echo "Checking roblox-rs-cli..."
cd roblox-rs-cli
cargo package --list
cargo publish --dry-run
cd ..

# Step 3: Actually publish the crates
echo "===== Step 3: Publishing crates ====="

if confirm "Ready to publish roblox-rs-core?"; then
  cd roblox-rs-core
  cargo publish
  cd ..
  echo "Waiting 15 seconds for crates.io to update..."
  sleep 15
else
  echo "Skipping roblox-rs-core publishing"
  exit 1
fi

if confirm "Ready to publish roblox-rs-ecs?"; then
  cd roblox-rs-ecs
  cargo publish
  cd ..
  echo "Waiting 15 seconds for crates.io to update..."
  sleep 15
else
  echo "Skipping roblox-rs-ecs publishing"
  exit 1
fi

if confirm "Ready to publish roblox-rs-cli?"; then
  cd roblox-rs-cli
  cargo publish
  cd ..
else
  echo "Skipping roblox-rs-cli publishing"
  exit 1
fi

echo "===== All crates published successfully! =====" 
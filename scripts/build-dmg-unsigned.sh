#!/bin/bash
# Build Unsigned DMG Script for Gõ Việt (GoxViet)
# For Homebrew distribution without Apple Developer Account

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

VERSION=$1
if [ -z "$VERSION" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: ./build-dmg-unsigned.sh <version>"
    echo "Example: ./build-dmg-unsigned.sh 1.0.0"
    exit 1
fi

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Gõ Việt (GoxViet) Unsigned Build v$VERSION${NC}"
echo -e "${GREEN}For Homebrew Distribution${NC}"
echo -e "${GREEN}========================================${NC}"

# Get project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo -e "\n${BLUE}Project root:${NC} $PROJECT_ROOT"
echo -e "${YELLOW}Note: This build is NOT code-signed or notarized${NC}"
echo -e "${YELLOW}Users will need to bypass Gatekeeper manually${NC}"

# Step 1: Clean previous builds
echo -e "\n${YELLOW}[1/6] Cleaning previous builds...${NC}"
cd "$PROJECT_ROOT/core"
cargo clean
echo -e "${GREEN}✓ Rust build cleaned${NC}"

cd "$PROJECT_ROOT/platforms/macos/goxviet"
if [ -d "build" ]; then
    rm -rf build
fi
if [ -d "dist" ]; then
    rm -rf dist
fi
xcodebuild clean -project goxviet.xcodeproj -scheme goxviet -configuration Release > /dev/null 2>&1 || true
echo -e "${GREEN}✓ Xcode build cleaned${NC}"


# Step 2: Build Rust core (universal binary)
echo -e "\n${YELLOW}[2/6] Building Rust core (universal binary for Apple Silicon and Intel)...${NC}"
cd "$PROJECT_ROOT/core"

# Build for Apple Silicon (arm64)
echo -e "${BLUE}Building for aarch64-apple-darwin...${NC}"
cargo build --release --target aarch64-apple-darwin

# Build for Intel (x86_64)
echo -e "${BLUE}Building for x86_64-apple-darwin...${NC}"
cargo build --release --target x86_64-apple-darwin

# Create universal binary
UNIVERSAL_LIB_PATH="$PROJECT_ROOT/platforms/macos/goxviet/libgoxviet_core.a"
LIPO_IN1="target/aarch64-apple-darwin/release/libgoxviet_core.a"
LIPO_IN2="target/x86_64-apple-darwin/release/libgoxviet_core.a"

if [ ! -f "$LIPO_IN1" ] || [ ! -f "$LIPO_IN2" ]; then
    echo -e "${RED}Error: One or both architecture static libraries not found${NC}"
    exit 1
fi

echo -e "${BLUE}Creating universal binary with lipo...${NC}"
mkdir -p "$PROJECT_ROOT/platforms/macos/goxviet"
lipo -create "$LIPO_IN1" "$LIPO_IN2" -output "$UNIVERSAL_LIB_PATH"

if [ ! -f "$UNIVERSAL_LIB_PATH" ]; then
    echo -e "${RED}Error: Universal libgoxviet_core.a not created${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Rust universal binary built successfully${NC}"

# Step 3: Build macOS app (unsigned)
echo -e "\n${YELLOW}[3/6] Building macOS application (unsigned)...${NC}"
cd "$PROJECT_ROOT/platforms/macos/goxviet"

mkdir -p build
mkdir -p dist

# Build using xcodebuild (Release configuration)
xcodebuild \
    -project goxviet.xcodeproj \
    -scheme goxviet \
    -configuration Release \
    -derivedDataPath build/DerivedData \
    CODE_SIGN_IDENTITY="" \
    CODE_SIGNING_REQUIRED=NO \
    CODE_SIGNING_ALLOWED=NO \
    | grep -E "^(Build|▸)" || true

# Find the built app
BUILT_APP=$(find build/DerivedData/Build/Products/Release -name "goxviet.app" -type d | head -1)

if [ -z "$BUILT_APP" ] || [ ! -d "$BUILT_APP" ]; then
    echo -e "${RED}Error: App bundle not found${NC}"
    exit 1
fi

# Copy to dist folder with branded name
cp -R "$BUILT_APP" dist/GoxViet.app
echo -e "${GREEN}✓ Application built successfully (unsigned)${NC}"

# Step 4: Prepare DMG directory
echo -e "\n${YELLOW}[4/6] Preparing DMG directory...${NC}"

DMG_DIR="$PROJECT_ROOT/platforms/macos/goxviet/dmg"
if [ -d "$DMG_DIR" ]; then
    rm -rf "$DMG_DIR"
fi
mkdir -p "$DMG_DIR"

# Copy app to DMG directory
cp -R dist/GoxViet.app "$DMG_DIR/"
echo -e "${GREEN}✓ App copied to DMG directory${NC}"

# Create Applications symlink
ln -s /Applications "$DMG_DIR/Applications"
echo -e "${GREEN}✓ Applications symlink created${NC}"

# Create README with Gatekeeper bypass instructions
cat > "$DMG_DIR/README.txt" <<EOF
Gõ Việt (GoxViet) v$VERSION
========================

IMPORTANT: Gatekeeper Bypass Required
======================================

This app is NOT code-signed or notarized by Apple.
You need to bypass Gatekeeper to run it.

Installation Steps:
===================

Method 1: Right-Click Open (Recommended)
-----------------------------------------
1. Drag "GoxViet.app" to Applications folder
2. Go to Applications folder
3. RIGHT-CLICK on "GoxViet.app"
4. Select "Open" from context menu
5. Click "Open" in the dialog that appears
6. Grant Accessibility permission when prompted

Method 2: Remove Quarantine (Advanced)
---------------------------------------
1. Drag "GoxViet.app" to Applications folder
2. Open Terminal and run:
   xattr -cr /Applications/GoxViet.app
3. Double-click the app to launch
4. Grant Accessibility permission when prompted

Method 3: System Settings (macOS 13+)
--------------------------------------
1. Drag "GoxViet.app" to Applications folder
2. Try to open the app (it will be blocked)
3. Go to: System Settings → Privacy & Security
4. Click "Open Anyway" next to the blocked app message
5. Grant Accessibility permission when prompted

After Installation:
===================
- Menu bar icon will appear (🇻🇳)
- Click icon to toggle Vietnamese/English
- Or use keyboard shortcut: Cmd+Shift+V
- Type "hoa" → "hòa" (with tone marks)

Requirements:
=============
- macOS 10.15 (Catalina) or later
- Accessibility permission (will be requested)

Support:
========
- GitHub: https://github.com/nihmtaho/goxviet
- Issues: https://github.com/nihmtaho/goxviet/issues

License: MIT

Note: For code-signed version, see official releases on GitHub.
EOF

echo -e "${GREEN}✓ README created with Gatekeeper bypass instructions${NC}"

# Step 5: Create DMG
echo -e "\n${YELLOW}[5/6] Creating DMG...${NC}"

DMG_NAME="GoxViet-$VERSION-unsigned.dmg"
DMG_PATH="$PROJECT_ROOT/platforms/macos/goxviet/dist/$DMG_NAME"
VOLUME_NAME="Gõ Việt $VERSION"

if [ -f "$DMG_PATH" ]; then
    rm "$DMG_PATH"
fi

# Create DMG with compression
hdiutil create -volname "$VOLUME_NAME" \
    -srcfolder "$DMG_DIR" \
    -ov -format UDZO \
    -imagekey zlib-level=9 \
    "$DMG_PATH" > /dev/null

echo -e "${GREEN}✓ DMG created successfully${NC}"

# Clean up DMG directory
rm -rf "$DMG_DIR"

# Step 6: Calculate checksums
echo -e "\n${YELLOW}[6/6] Calculating checksums...${NC}"

cd "$(dirname "$DMG_PATH")"
SHA256=$(shasum -a 256 "$DMG_NAME" | cut -d' ' -f1)
echo "$SHA256  $DMG_NAME" > "$DMG_NAME.sha256"

echo -e "${GREEN}✓ Checksums calculated${NC}"

# Get file size
DMG_SIZE=$(du -sh "$DMG_PATH" | cut -f1)

# Summary
echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}Build Complete!${NC}"
echo -e "${GREEN}========================================${NC}"

echo -e "\n${BLUE}Output Files:${NC}"
echo -e "  DMG:      ${YELLOW}$DMG_NAME${NC}"
echo -e "  Path:     ${YELLOW}$DMG_PATH${NC}"
echo -e "  Size:     ${YELLOW}$DMG_SIZE${NC}"
echo -e "  SHA-256:  ${YELLOW}$SHA256${NC}"

echo -e "\n${BLUE}Checksum File:${NC}"
echo -e "  ${YELLOW}$DMG_NAME.sha256${NC}"

echo -e "\n${YELLOW}⚠️  Important Notes:${NC}"
echo -e "  • This DMG is NOT code-signed or notarized"
echo -e "  • Users must bypass Gatekeeper manually (see README.txt in DMG)"
echo -e "  • Suitable for Homebrew distribution"
echo -e "  • NOT suitable for App Store distribution"

echo -e "\n${YELLOW}Next Steps:${NC}"
echo -e "  1. Test DMG installation:"
echo -e "     ${BLUE}open $DMG_PATH${NC}"
echo -e ""
echo -e "  2. Test Gatekeeper bypass:"
echo -e "     ${BLUE}# Drag to Applications, then:${NC}"
echo -e "     ${BLUE}xattr -cr /Applications/GoxViet.app${NC}"
echo -e "     ${BLUE}open /Applications/GoxViet.app${NC}"
echo -e ""
echo -e "  3. Upload to GitHub Release"
echo -e ""
echo -e "  4. Create Homebrew Cask:"
echo -e "     ${BLUE}./scripts/create-homebrew-cask.sh $VERSION${NC}"
echo -e ""
echo -e "  5. Test Homebrew installation:"
echo -e "     ${BLUE}brew install --cask goxviet${NC}"

echo -e "\n${GREEN}Done!${NC}"

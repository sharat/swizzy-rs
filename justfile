# Read version from Cargo.toml
version := `grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'`

# Tag and push current Cargo.toml version to trigger the release workflow
publish:
    @echo "Publishing v{{version}}..."
    @git diff --exit-code Cargo.toml > /dev/null || (echo "Cargo.toml has uncommitted changes — commit first" && exit 1)
    @git tag --list "v{{version}}" | grep -q . && (echo "Tag v{{version}} already exists" && exit 1) || true
    git tag -a "v{{version}}" -m "Release v{{version}}"
    git push origin "v{{version}}"
    @echo "Pushed v{{version}} — release workflow will start automatically"

[![crates.io](https://img.shields.io/crates/v/fuzzypath.svg)](https://crates.io/crates/fuzzypath)

[![docs.rs](https://docs.rs/fuzzypath/badge.svg)](https://docs.rs/fuzzypath)

# ๐งน `fuzzypath`

Quick & dirty fuzzy path comparison

# Comparison rules

* โ Case insensitive
* โ Backslashes are normalized to forward slashes
* โ Trailing slashes are removed, except for root slash (for absolute POSIX paths)
* โ Repeating slashes are normalized to a single slash
* โ Comparing a Windows path with a POSIX path will not work if either is absolute (Windows paths with a drive letter, POSIX paths with a preceeding slash)
* โ Comparing a Windows UNC path will not work with any POSIX path
* โ POSIX paths can contain backslashes in file names, but Windows paths cannot - these will be normalized to forward slashes and you will lose that information

# Usage

Add `fuzzypath` to your Cargo.toml dependencies:

```toml
[dependencies]
fuzzypath = "1"
```

# Serde

To enable Serde serialization and deserialization, use the crate feature `serde`

```toml
[dependencies]
fuzzypath = { version = "1", features = ["serde"] }
```
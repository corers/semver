# Core Semver

[Semantic Versioning](https://semver.org/) is a guideline for how version numbers are assigned and incremented. This crate is based on [semver](https://docs.rs/semver) and has some extended functions, such as **bump version**.

## Installation

```bash
cargo add core_semver
```

## Usage

> [!TIP]
> For more information, see the [rust doc](https://docs.rs/core_semver/).

```rust
use crate::core_semver::parse;

fn main() {
  // prase version str
  // return struct -> Version
  let mut version = core_semver::parse("1.2.3-beta.4");
  assert_eq!(version.major, 1);
  assert_eq!(version.minor, 2);
  assert_eq!(version.patch, 3);
  assert_eq!(version.pre.as_str(), "beta.4");
  assert_eq!(version.to_string(), "1.2.3-beta.4");

  // bump version
  assert_eq!(version.bump_pre(None), "1.2.3-beta.5");
  assert_eq!(version.bump_pre(Some("rc")), "1.2.3-rc.1");
  assert_eq!(version.bump_patch(), "1.2.4");
  assert_eq!(version.bump_minor(), "1.3.0");
  assert_eq!(version.bump_major(), "2.0.0");
}
```

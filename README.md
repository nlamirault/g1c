# g1c - Google Cloud Instances Terminal UI Manager

[![GitHub Release](https://img.shields.io/github/release/nlamirault/g1c.svg?logo=github)](https://github.com/nlamirault/g1c/releases)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/nlamirault/g1c/blob/main/LICENSE)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/nlamirault/g1c/badge)](https://scorecard.dev/viewer/?uri=github.com/nlamirault/g1c)
[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/10711/badge)](https://bestpractices.coreinfrastructure.org/projects/10711)

A terminal user interface for monitoring and managing Google Cloud Instances, inspired by
[k9s](https://github.com/derailed/k9s) for Kubernetes, [e1s](https://github.com/keidarcy/e1s) for AWS EC2.

![g1c Screenshot](assets/main-view.png)

## Features

- 🖥️ Real-time monitoring of Google Cloud VM instances
- 🔍 Filter and search instances by various attributes
- ⚡ Perform common operations (start, stop, restart, etc.)
- 🔄 Auto-refresh with configurable intervals
- 🌍 Support for multiple projects and regions
- 🔐 Authentication using Google Cloud credentials
- 🎨 Customizable UI with different color themes

## Prerequisites

- Rust and Cargo installed (version 1.75.0 or higher recommended)
- Google Cloud SDK (gcloud CLI) installed and configured
- Active Google Cloud account with appropriate permissions
- A Google Cloud project with Compute Engine API enabled

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/nlamirault/g1c.git
cd g1c

# Build and install
make install
```

### Using Cargo

```bash
cargo install g1c
```

## Usage

```bash
# View instances from default configured project
g1c

# View instances from a specific project
g1c --project my-project-id

# View instances from a specific region
g1c --region us-central1   # or -g us-central1

# View instances with refresh interval of 5 seconds
g1c --refresh 5   # or -r 5
```

## Keyboard Shortcuts

| Key                | Action                    |
| ------------------ | ------------------------- |
| `q`, `Ctrl+c`      | Quit                      |
| `?`                | Show help                 |
| `↑`, `↓`, `j`, `k` | Navigate up/down          |
| `Enter`            | Show instance details     |
| `f`                | Filter instances          |
| `r`                | Refresh data              |
| `s`                | Start instance            |
| `S`                | Stop instance             |
| `R`                | Restart instance          |
| `d`                | Delete instance           |
| `/`                | Search                    |
| `ESC`              | Close popup/cancel action |

## Configuration

g1c will look for configuration in the following locations:

1. Command line arguments
2. Environment variables
3. Configuration file at `~/.config/g1c/config.toml`

Example configuration file:

```toml
default_project = "my-project-id"
default_region = "us-central1"
refresh_interval = 3 # seconds
theme = "dark"
```

## Authentication

g1c uses the standard Google Cloud authentication methods through the gcloud CLI. Make sure you have:

1. Google Cloud SDK installed and configured
2. Authenticated with `gcloud auth login`
3. Set a default project with `gcloud config set project PROJECT_ID` or use the `--project` flag

You can check your authentication status with:

```bash
make check-env
```

## SLSA

All _artifacts_ provided by this repository meet [SLSA L3](https://slsa.dev/spec/v1.0/levels#build-l3)

### Verify SLSA provenance

Using the [Github CLI]():

```shell
$ gh attestation verify --owner nlamirault e2c_darwin_arm64_v0.1.2
gh attestation verify --owner nlamirault g1c_0.1.16_darwin_arm64.sbom.json
Loaded digest sha256:b8a7adf91b122b488118d50cddfb808cd161ba04f989de5a5f434f5fe85773d3 for file://g1c_0.1.16_darwin_arm64.sbom.json
Loaded 1 attestation from GitHub API
✓ Verification succeeded!

sha256:b8a7adf91b122b488118d50cddfb808cd161ba04f989de5a5f434f5fe85773d3 was attested by:
REPO            PREDICATE_TYPE                  WORKFLOW
nlamirault/g1c  https://slsa.dev/provenance/v1  .github/workflows/release.yml@refs/tags/v0.1.16
```

## SBOM

You could use [trivy](https://trivy.dev) to read SBOM file:

```shell
$ trivy sbom
2025-06-09T16:57:57+02:00       INFO     [vuln] Vulnerability scanning is enabled
2025-06-09T16:57:57+02:00       INFO     Detected SBOM format    format="spdx-json"
2025-06-09T16:57:57+02:00       INFO     Number of language-specific files       num=0
```

or with [grype](https://github.com/anchore/grype):

```shell
$ cat ./g1c_0.1.16_darwin_arm64.sbom.json | grype
No vulnerabilities found
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Apache Version 2.0

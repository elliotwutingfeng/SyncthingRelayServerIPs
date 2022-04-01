# Syncthing Relay Server IP Addresses

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/GitHub_Actions-2088FF?style=for-the-badge&logo=github-actions&logoColor=white)

[![GitHub license](https://img.shields.io/badge/LICENSE-BSD--3--CLAUSE-GREEN?style=for-the-badge)](LICENSE)
[![scraper](https://img.shields.io/github/workflow/status/elliotwutingfeng/SyncthingRelayServerIPs/scraper?label=SCRAPER&style=for-the-badge)](https://github.com/elliotwutingfeng/SyncthingRelayServerIPs/actions/workflows/scraper.yml)
<img src="https://img.shields.io/tokei/lines/github/elliotwutingfeng/SyncthingRelayServerIPs?label=Total%20Allowlist%20IPs&style=for-the-badge" alt="Total Allowlist IPs"/>

Machine-readable `.txt` allowlist of [Syncthing Relay Server IP Addresses](https://relays.syncthing.net), updated every hour.

**Disclaimer:** _This project is not sponsored, endorsed, or otherwise affiliated with Syncthing._

## Allowlist download
You may download the allowlist [here](ips.txt?raw=1)

## Requirements

-   Rust >= 1.59.0

## Setup instructions

`git clone` and `cd` into the project directory, then run the following

```bash
cargo build
```

## Usage

```bash
cargo run
```

&nbsp;

<sup>These files are provided "AS IS", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, arising from, out of or in connection with the files or the use of the files.</sup>

<sub>Any and all trademarks are the property of their respective owners.</sub>

# Introduction

A CLI tool for MyAnimeList, written in rust.

![CleanShot 2024-01-13 at 12 21 51](https://github.com/rkaahean/mal-cli/assets/16059999/3153e17a-07dd-487d-b6b7-0885230f896b)

# Installation

```bash
cargo install mal-cli
```

# Setup

1. You will need your own client ID. You you get one [here](https://myanimelist.net/apiconfig/create).
   - The app redirect URL needs to be `http://localhost:8080`. Everything else can be custom.
   - Example ![CleanShot 2024-01-13 at 12 28 58@2x](https://github.com/rkaahean/mal-cli/assets/16059999/b32e4701-3360-4f22-9cdb-45d93caceb41)
2. Export the client ID as an env.
   ```bash
   export MAL_CLI_CLIENT_ID=
   ```

# Usage

### `list`
Shows your MyAnimeList. Ex: `mal-cli list`. You can add an additional parameter `--num` to filter for the number of anime returned.

### `season`
Shows the anime for a given season and year. 
Ex: `mal-cli season --season=fall --year=2022`
   


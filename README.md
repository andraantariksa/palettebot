# PaletteBot

PaletteBot (https://www.facebook.com/palettebot/) clone, but in Rust

This repository is under development and only for experimental use only.

If you're interested with this project, please let me know :) !

# Usage

First, you need to get setup the media you want to post and API keys

Build the file

```
cargo build --release
```

## Setup the Crontab file

To make the process run automated, we can make a cron job. Simply modify `/etc/crontab` file by

```
sudo nano /etc/crontab
```

then append this command

```
*/15 * * * * "cd /home/YOUR_USER/palettebot && ./target/release/palettebot" >/dev/null 2>&1
```

\*/15 means we are executing the command for every 15 minutes. Take a look at [Crontab Generator - Generate crontab syntax](https://crontab-generator.org).

# Media

- Facebook - You have to get your page never expire `access_token`. [How to Generate a Never Expiring Facebook Page Access Token](https://medium.com/@yasithlokuge/how-to-generate-a-never-expiring-facebook-page-access-token-24ac5c1a95f1)

# API Keys

- [Pexels](https://www.pexels.com/api/new/)
- [Unsplash](https://unsplash.com/oauth/applications)

Don't forget to read their term and rules! Do not violate it!

# Lisence

GNU General Public License v3.0
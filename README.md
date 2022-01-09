# HoneyUp

## An uploader honeypot designed to look like poor website security.

### Requirements

- Linux server
- NGiNX
- Rust toolchain (build only)

### Installation

1. Build using `cargo build --release`.
2. Log into your server and create a `honeyup` user with a home directory.
3. Upload/copy the contents of this repo to your honeyup user's home `/home/honeyup`.
4. Copy the `honeyup` binary from `target/release/` to `/home/honeyup/`.
5. Edit `res/config.env.txt` to make it believable (add some canary tokens too).
6. Upload/copy the systemd service descriptor from `conf/honeyup.service` to `/etc/systemd/system/honeyup.service`.
7. Reload all systemd service descriptors `systemctl daemon-reload`.
8. Ensure any currently served sites do not use the `/uploads` path.
9. Add the contents of `conf/nginx_snippet.conf` to your NGiNX config just above your server's `location` blocks.
10. Reload the NGiNX config with `systemctl reload nginx`.
11. Enable and start the `honeyup` service `systemctl enable honeyup` & `systemctl start honeyup`.
12. Browse to `/uploads` on your website and use `upload.php` to upload some dummy/bait files. 

### Notes

Be sure to check up on the `uploaded_files` folder often to be sure you aren't hosting somebody's malware.

Also if you are using Docker, please be sure to change `/path/to/uploaded_files` in the docker-compose.yml before building.
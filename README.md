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
6. Add a copy of your favorite Windows calc.exe to the `res` folder
7. Upload/copy the systemd service descriptor from `conf/honeyup.service` to `/etc/systemd/system/honeyup.service`.
8. Reload all systemd service descriptors `systemctl daemon-reload`.
9. Ensure any currently served sites do not use the `/uploads` path.
10. Add the contents of `conf/nginx_snippet.conf` to your NGiNX config just above your server's `location` blocks.
11. Reload the NGiNX config with `systemctl reload nginx`.
12. Enable and start the `honeyup` service `systemctl enable honeyup` & `systemctl start honeyup`.
13. Browse to `/uploads` on your website and use `upload.php` to upload some dummy/bait files. 

### Notes

Be sure to check up on the `uploaded_files` folder often to be sure you aren't hosting somebody's malware.

### Docker Installation

You will need Docker and docker-compose for this

1. Clone the repository `git clone https://github.com/LogoiLab/honeyup.git`
2. The docker-compose.yml file has a couple of environment variables you will want to set. The `APP_ID`, `AWS_ACCESS_KEY_ID`, and `AWS_SECRET_ACCESS_KEY` will be set with a service like [Canary Tokens](https://canarytokens.org/generate). The `SMTP`, `ADDRESS`, and `PASSWORD` variables are used to make your honeyup look more reputable as a vulnerable server. Finally, please set the `/path/to/uploaded_files` as a location outside the container where you would like the uploaded files to be stored
3. Run `docker-compose up -d` in the honeyup directory
4. Done!

The container will listen on port 4000 unless specified otherwise.

FIFO Tunnel
===========
This app open a Linux FIFO file and try to send the info to another Linux FIFO file, but
without blocking on write. This allow to create an async tunnel between 2 FIFO files.

The inspiration for this app was that some apps (like ffmpeg) don't have any method to
rotate some information (like `--vstats_file` of ffmpeg) generated on files.

== Usage
It's simple:
```sh
app fifo_file_in fifo_file_out
```
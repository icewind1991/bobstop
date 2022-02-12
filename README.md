# bobstop

Pause viewmodel bobbing in a demo until a specific tick.

## Why

While tf2's viewmodel bobbing is deterministic on the surface, variations in framerate can lead to small differences
between multiple playbacks of the same demo file.
This can lead to issues when combining multiple recording passes of a demo file in a video.

## What

This aims to minimize the inconsistencies in viewmodel bobbing by pausing the bobbing until the beginning of the clip
recording, removing additive inconsistencies in viewmodel positioning from seeking to the beginning of the clip.

This is done by inserting commands in the demo file to change the "bobbing cycle".

## How

```bash
bobstop.exe <demofile> <start_tick>
```

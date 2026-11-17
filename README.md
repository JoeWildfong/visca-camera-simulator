# VISCA camera simulator

This repo is intended to help test [esmc-camera].

It receives commands over TCP and provides a GUI to manually send accept/completion messages back.
This can be used to simulate a real VISCA camera, but it has not been verified to match any real behaviour.

## Testing setup

Clone both this repo and [esmc-camera].
Run both with `npm run tauri dev`.

In the esmc-camera UI, select TCP (Debug) as the camera to control.
If the connection succeeds, this app's UI borders will change from red to green.

[esmc-camera]: https://github.com/JoeWildfong/esmc-camera

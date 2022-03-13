# quad-snd

**If you are into simple and minimalistic game development and you only want to play prerecorded files, this fork wont be of use to you - checkout the original quad-snd project instead (link below)**

This is a fork of [not-fl3's](https://github.com/not-fl3) [quad-snd](https://github.com/not-fl3/quad-snd) a high-level, light-weight, and opinionated audio library,
but this forks goal is to remove all the high-level mixer functionality, instead replacing it with a low-level manual buffer-callback approach.

See the Goals section for more information

## Goals
- remove high-level code, including sound file loading mechanism
- be a low-level approach
- fill a buffer using a callback
- structure similar to the one used by `rust-sdl2's` audio subsystem 

## Support
This fork is still in its early stages,
and even though the original `quad-snd` library has support for all the following platforms,
it will take a while until that level of support has been achived.

- [ ] Web: WebAudio (impossible as of now)
- [x] Android: OpenSLES (checks done, untested)
- [x] Linux: Alsa (checks done, tested)
- [ ] macOS: CoreAudio (error)
- [x] Windows: Wasapi (checks done, untested)
- [ ] iOS: CoreAudio (error)

## Attribution
Some Attribution taken from the original repository

- https://github.com/floooh/sokol/blob/master/sokol_audio.h
- https://github.com/norse-rs/audir
- https://github.com/unrust/uni-snd

And obviously a huge shoutout to the original repository https://github.com/not-fl3/quad-snd

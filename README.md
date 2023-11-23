# frame-translation-generator
**frame-translation-generator** is a simple command line tool written in Rust. It generates MoreMcmeta animations by 
translating a window across an image to generate an animation. This is useful for generating "sliding" animations like 
the Minecraft enchantment glint.

## Options
Run the tool with the `--help` flag to view a list of the tool's options.

* `input` - path to image to use as input
* `output` - path where new file will be written. Any existing file at this location will be overwritten.
* `frame-width` - width of the window and each frame in the resulting animation. This is not the width of the entire image.
* `frame-height` - height of the window and each frame in the resulting animation. This is not the height of the entire image.
* `x-start` - initial x-coordinate of the top-left corner of the window. x=0 refers to the left side of the image.
* `y-start` - initial y-coordinate of the top-left corner of the window. y=0 refers to the top of the image.
* `dx` - horizontal pixels by which to translate the window each frame
* `dy` - horizontal pixels by which to translate the window each frame
* `max-frames` - maximum number of frames to generate

For example,
```shell
$ ./frame-translation-generator --input=myimage.png --frame-width=64 --frame-height=64 --dx=1 --dy="-3" --x-start=0 --y-start=4032 --max-frames=1024 --output=output.png`
```
will read `myimage.png`, put the first 64x64 frame at (0, 4032), and then generate frames by moving the window right 1 
1 pixel and up 3 pixels until the image boundary is reached or until 1024 frames are generated. The frames will be 
written to `output.png`.

## Building the Tool
[Install Rust](https://www.rust-lang.org/tools/install). Then run `cargo build` or `cargo build --release` (for an 
optimized build) from the command line.

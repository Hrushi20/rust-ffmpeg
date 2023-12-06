# Rust SDK for FFmpeg using WasmEdge Plugin.

## Introduction

* **Easy to use**: Simple APIs to process Audio, Video files.
* **Improved Performance**: Computation done on Host devices providing near native speed.
* **Compatability**: SDK is similar to [rust-ffmpeg](https://github.com/zmwangx/rust-ffmpeg) crate. Hassle free integration.
* **Version**: Supports FFmpeg [v6.0](https://github.com/FFmpeg/FFmpeg/tree/release/6.0)

### Status
* [x] AVCodec 
* [ ] AVDevice 
* [x] AVFilter 
* [x] AVFormat 
* [x] SWResample
* [x] SWScaling
* [x] AVUtil 

## Examples

### Chapters
``` console 
 $ cargo run --release --example chapters ./assets/bunny.mp4 
    [mov,mp4,m4a,3gp,3g2,mj2 @ 0x144fd9ad0] Referenced QT chapter track not found
    Nb chapters: 1
    chapter id 0:
            time_base: 1/10000000
            start: 0
            end: 20000000
            title: 
    Added chapter with id 0 to output

    Ouput: nb chapters: 1
    chapter id 0:
            time_base: 1/10000000
            start: 0
            end: 20000000
            title: 
```

### Codec Info

The below example is for hdr decoder/encoder

```console
$ cargo run --release --example codec-info --  hdr
    type: decoder
             id: RADIANCE_HDR
             name: hdr
             description: HDR (Radiance RGBE format) image
             medium: Video
             capabilities: DR1 | FRAME_THREADS
             rates: any
             formats: any
             max_lowres: 0

    type: encoder
             id: RADIANCE_HDR
             name: hdr
             description: HDR (Radiance RGBE format) image
             medium: Video
             capabilities: DR1 | FRAME_THREADS
             rates: any
             formats: [GBRPF32LE]
             max_lowres: 0
```

### Dump Frames
``` console
$ cargo run --release --example dump-frames ./assets/bunny.mp4 
```

### Metadata
``` console
$ cargo run --release --example metadata ./assets/bunny.mp4
    major_brand: isom
minor_version: 512
compatible_brands: isomiso2avc1mp41
title: Adding Chapters using FFMPEG
artist: Hrushi20
encoder: Lavf60.10.100
Best video stream index: 0
Best audio stream index: 1
duration (seconds): 2.00
stream index 0:
        time_base: 1/15360
        start_time: 0
        duration (stream timebase): 30720
        duration (seconds): 2.00
        frames: 120
        disposition: DEFAULT
        discard: Default
        rate: 60/1
        medium: Video
        id: H264
        bit_rate: 4773892
        max_rate: 0
        delay: 0
        video.width: 1920
        video.height: 1080
        video.format: YUV420P
        video.has_b_frames: true
        video.aspect_ratio: 1/1
        video.color_space: Unspecified
        video.color_range: Unspecified
        video.color_primaries: Unspecified
        video.color_transfer_characteristic: Unspecified
        video.chroma_location: Left
        video.references: 1
        video.intra_dc_precision: 0
stream index 1:
        time_base: 1/48000
        start_time: 0
        duration (stream timebase): 96000
        duration (seconds): 2.00
        frames: 95
        disposition: DEFAULT
        discard: Default
        rate: 0/0
        medium: Audio
        id: AAC
        bit_rate: 391959
        max_rate: 0
        delay: 0
        audio.rate: 48000
        audio.channels: 6
        audio.format: F32(Planar)
        audio.frames: 0
        audio.align: 0
        audio.channel_layout: (empty)
```

### Remux 
```console
$ cargo run --release --example remux ./assets/bunny.mp4 ./assets/bunny.mkv
    [mov,mp4,m4a,3gp,3g2,mj2 @ 0x152ed6890] Referenced QT chapter track not found
```

## Related Links

- [WasmEdge](https://github.com/WasmEdge/WasmEdge)
- [FFmpeg](https://github.com/FFmpeg/FFmpeg)
- [WasmEdge FFmpeg Plugin]()

## Notice
This work is made possible by **[Zmwangx](https://github.com/zmwangx/rust-ffmpeg) work on [rust-ffmpeg](https://github.com/zmwangx/rust-ffmpeg)**.

## License
TODO

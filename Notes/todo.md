# music visualizer

## 03/27

today we caught-up and poked around the current code base...

- id3
    > _way more_ than you ever thought there was to know about `id3` tags
  - [ID3.org](https://id3.org/Home)
  - [id3 - Rust](https://docs.rs/id3/latest/id3/)
- formats/playback
    > we should discuss as I think I'm finding the same pkgs that you did
  - [RustAudio/rodio: Rust audio playback library](https://github.com/RustAudio/rodio?tab=readme-ov-file)
    - [rodio docs](https://docs.rs/rodio/latest/rodio/index.html)
  - [RustAudio/cpal: Cross-platform audio I/O library in pure Rust](https://github.com/RustAudio/cpal)
  - [Symphonia: Pure Rust multimedia format demuxing, **tag reading**, and audio decoding library](https://github.com/pdeljanov/Symphonia/)
- mvc
  > a simple [mvc](https://www.cs.sjsu.edu/~pearce/modules/lectures/ood2/mvc/index.htm) diagram that might help to remember:
  - **model** data can be of type `int`/`float` and represent colors, 2d shapes, 3d shapes, _etc._ and rendered by a **view** as an image  
  - that doesn't preclude a **model** of type `int`/`float` from representing audio samples (vs. colors) and rendered by a **view** (audio vs. image) as [American Pie](https://music.apple.com/us/song/american-pie-full-length-version/1440834619)  
    ![american pie](https://upload.wikimedia.org/wikipedia/en/thumb/b/b9/American_Pie_by_Don_McLean.png/250px-American_Pie_by_Don_McLean.png)
- performance
    > after you absorb the rendering of your audio in multiple formats: music, color, shape, _etc._, you might want to profile your code 😎
  - [Profiling - The Rust Performance Book](https://nnethercote.github.io/perf-book/profiling.html)
  - [Benchmark tests](https://doc.rust-lang.org/1.4.0/book/benchmark-tests.html)
  - [test - The Rust Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html)
- ui/ux
    > look what I found❗
  - [conway.drawio](https://app.diagrams.net/?src=about)

## 03/27 next

- review above; id3, formats/playback...
- measurement of what's taking so long on startup
- create your own `conway.drawio`

good luck and have fun!

## How to Download any Song as .wav from Youtube

### Step 1:
1. Go to Youtube
2. Search for the song you want to download
3. Copy song url from the address bar

### Step 2:
1. Go to [Yout.com](https://yout.com/)
2. Paste Youtube url
3. Select .wav and click download

### Step 3:
1. Move downloaded .wav to music_library
2. open cmd prompt to cargo.toml level
3. run `ffmpeg -i music_library/input_file.wav -ar 44100 -ac 2 -sample_fmt s16 music_library/output_file.wav`
    
    - Ex: `ffmpeg -i music_library/american-pie.wav -ar 44100 -ac 2 -sample_fmt s16 music_library/american-pie-fixed.wav`

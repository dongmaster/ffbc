# ```FF```mpeg ```B```atch ```C```onverter
FFBC is a program that uses ffmpeg to batch (mass) convert contents of directories recursively.

Confirmed working on Linux. Haven't tried Windows yet but it should work.

## Usage
```
./ffbc -h

Usage: ./ffbc [DIRECTORY] [OLD_EXT] [NEW_EXT]

[OLD_EXT] and [NEW_EXT] are file extensions.

USAGE
    ./ffbc some_directory flac mp3
    The first argument that gets passed is the target directory (some_directory).
    The second argument that gets passed is the current file extension you want to convert FROM (flac).
    The third argument that gets passed is the new file extension you want to convert TO (mp3).
    
    This will convert everything in that directory (recursively, so directories in the first directory will also be targeted).
    It targets flac files and converts them to mp3 using ffmpeg.
    
    You can also use the -c argument to pass some options to ffmpeg, like -b:a 320k to set the audio quality to 320kbps.
    ./ffbc -c -b:a 320k
    ./ffbc directory wav ogg
    
    The file formats you can convert from and to is limited by ffmpeg.
    The original files will NOT be replaced.
    
    Technically, you're supposed to be able to be able to supply the program with several directories and it will work.
    I haven't tried this though. ./ffbc dir1 wav flac dir2 wav flac dir3 mkv mp4

OPTIONS
    -h, --help
        Prints out this help message.
        
    -c, --config
        Changes the ffmpeg command-line arguments,
        See example 1,

EXAMPLES
    Example 1
        ./ffbc -c b:a 320k
        This will set the audio quality to 320kbps.
        
        After you've set that option, do this: ./ffbc some_directory flac mp3
```

## Dependencies
You need to have ffmpeg installed and have it in your $PATH.

## Compiling
Install the latest Rust Nightly (http://rust-lang.org select other Other Downloads) and then:
```git clone https://github.com/dongmaster/ffbc && cd ffbc && cargo build --release```

The resulting binary will be in ffbc/target/release/ffbc

## Todo

Create a directory inside the directory you want to convert and put the converted files in that new directory.

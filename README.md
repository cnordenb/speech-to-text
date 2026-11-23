[![GitHub last commit](https://img.shields.io/github/last-commit/cnordenb/speech-to-text)](#)
[![language](https://img.shields.io/badge/language-Rust-DEA584)](#)
[![license](https://img.shields.io/badge/license-MIT-007FFF)](LICENSE)
[![testing](https://img.shields.io/badge/testing-yes-C3FF00)](documentation/TESTS.md)
[![maintainable](https://img.shields.io/badge/maintainable-yes-00FF72)](documentation/MAINTAINER.md)

# speech-to-text

## But why?
Your client probably isn't paying you to spend time transcribing their voice messages.

## Description

An application for Windows and Linux that transcribes audio files, such as .ogg, to text that can be copied or exported
as .md or .txt, in a window or from the command line.

## Usage

Start `speech-to-text.exe` on Windows, or `speech-to-text` on Linux; nothing is installed, and no internet connection
is needed, so the audio never leaves the computer. It runs on Windows 10 and 11, 64-bit, and on 64-bit Linux.
[MAINTAINER.md](documentation/MAINTAINER.md) says how the executable is built.

### In the window

1. Press **Open audio file…**, or drop an audio file on the window: `.ogg` (Vorbis or Opus, such as a voice note),
   `.wav`, `.mp3`, `.flac`, `.m4a` or `.aac`. A file dropped on the executable's icon, or opened with it from
   Explorer's "Open with", is transcribed at once.
2. Wait while it is transcribed. The language is recognised by itself. **Model** chooses between `tiny` (fastest),
   `base` and `small` (most accurate, and the one chosen at the start).
3. Read, correct or select the text in the **Transcript** field. **Copy** puts all of it on the clipboard.
4. **Export as .txt** or **Export as .md** saves it as a file.

### From the command line

For scripts and AI agents, the same executable transcribes in the terminal:

```
speech-to-text transcribe "voice note.ogg"
speech-to-text transcribe "voice note.ogg" --model tiny --format md --output "voice note.md"
speech-to-text --help
```

The transcript goes to standard output, or to the file named by `--output`. `--model` is `tiny`, `base` or `small`
(the default), `--format` is `txt` (the default) or `md`. The exit status is 0 when the transcript was written, 1 on a
failure, such as "No speech was found in the audio file.", and 2 when the arguments were wrong.

The executable is large, about 1.4 GB, because the three models are built into it.

## Contributing

Anyone is welcome to open an [issue](https://github.com/cnordenb/speech-to-text/issues) to submit a bug report or suggest a new feature. Alternatively, you may fork or copy the repository and take it in any direction you wish.

If you wish to contribute work into this repository, please consult [CONTRIBUTING.md](.github/CONTRIBUTING.md) before any commits or pull requests.

## Maintaining

To keep speech-to-text working and up to date, see [MAINTAINER.md](documentation/MAINTAINER.md).

## Acknowledgements

Development of speech-to-text was accelerated with Claude Fable 5.2 and Claude Opus 5.5.

[![GitHub last commit](https://img.shields.io/github/last-commit/cnordenb/speech-to-text)](#)
[![language](https://img.shields.io/badge/language-Rust-DEA584)](#)
[![license](https://img.shields.io/badge/license-MIT-007FFF)](LICENSE)
[![testing](https://img.shields.io/badge/testing-yes-C3FF00)](documentation/TESTS.md)
[![maintainable](https://img.shields.io/badge/maintainable-yes-00FF72)](documentation/MAINTAINER.md)

# speech-to-text

## But why?
Your client probably isn't paying you to spend time transcribing their voice messages.

## Description

A Windows application that transcribes audio files, such as .ogg, to text that can be copied or exported as .md or .txt.

## Usage

Start `speech-to-text.exe`; nothing is installed, and no internet connection is needed. It runs on Windows 10 and
11, 64-bit. [MAINTAINER.md](documentation/MAINTAINER.md) says how the executable is built.

1. Press **Open audio file…**, or drop an audio file on the window: `.ogg` (Vorbis or Opus, such as a voice note),
   `.wav`, `.mp3`, `.flac`, `.m4a` or `.aac`. A file dropped on the executable's icon, or opened with it from
   Explorer's "Open with", is transcribed at once.
2. Wait while it is transcribed. The language is recognised by itself. **Model** chooses between `tiny` (fastest),
   `base` and `small` (most accurate, and the one chosen at the start).
3. Read, correct or select the text in the **Transcript** field. **Copy** puts all of it on the clipboard.
4. **Export as .txt** or **Export as .md** saves it as a file.

The executable is large, about 1.4 GB, because the three models are built into it.

## Contributing

Anyone is welcome to open an [issue](https://github.com/cnordenb/speech-to-text/issues) to submit a bug report or suggest a new feature. Alternatively, you may fork or copy the repository and take it in any direction you wish.

If you wish to contribute work into this repository, please consult [CONTRIBUTING.md](.github/CONTRIBUTING.md) before any commits or pull requests.

## Maintaining

To keep speech-to-text working and up to date, see [MAINTAINER.md](documentation/MAINTAINER.md).

## Acknowledgements

Development of speech-to-text was accelerated with Claude Fable 5.2.

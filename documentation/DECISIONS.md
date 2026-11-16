# Decisions

- [Rules, and how they are read](#rules-and-how-they-are-read)
- [Requirements, and how they are read](#requirements-and-how-they-are-read)
- [Code and files](#code-and-files)
- [Tests](#tests)
- [Documents](#documents)

The decisions recorded in the development log in [DEVLOG/](DEVLOG/DEVLOG1.md), by subject (Rule 23). Each ends with the
day it was made, and the time where the log has one, as a link to its entry in the log. Where a later decision replaced
an earlier one, the earlier one is named after it, with its own link.

## Rules, and how they are read

- **`.rs` and `.toml` files are code; `Cargo.lock` is not.** Rule 24 leaves out `*.lock` files, as it does `*.json`, so
  Rules 12 and 13 do not measure it.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
- **Rules have tests.** Compliance with a rule is one of the purposes Rule 5 names: Tests 16 and 17 enforce Rules 10 and
  11. [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
- **No test for a tool's sake.** A mutant that survives, or a line that no test runs, gets a test only if a requirement,
  a rule or a bug that was found asks for one.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **The build script has a folder of its own, `prebuild/`.** The main folder holds its eight files with `Cargo.toml` and
  `Cargo.lock` (Rule 14), and crapper, dryer and mutator pass over a folder named `build`.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **A test is written only for a purpose Rule 5 names:** compliance with a requirement or a rule, or catching again a
  bug found before. There is no limit on the number of tests.
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
- **Rule 2 names the chief developer in full,** as `LICENSE` does, beside the name on GitHub.
  [2026-11-16 17:10:24](DEVLOG/DEVLOG1.md#171024-the-project-set-up-from-the-template)
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)

## Requirements, and how they are read

- **`REQUIREMENTS.md` holds the seven requirements proposed from the request,** and `TESTS.md` cites them by number.
  Replaced: the request standing in for `REQUIREMENTS.md`, cited as "proposed Req.".
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
- **"Rust only" covers the dependencies too.** All of the project's code is Rust, and so is everything built into the
  executable; that is why Whisper runs on candle, and Opus on `opus-decoder`.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **Three models, chosen in the window and built into the executable:** tiny, base and small, with small chosen at the
  start, which makes the executable 1.43 GB. `medium` would take it past what Windows runs.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
- **Opus counts as .ogg.** Voice notes are Opus in an Ogg file, so both Vorbis and Opus are decoded.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **Other audio files are taken too:** .wav, .mp3, .flac and .m4a, by symphonia.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **The language is recognised, not asked for,** from the first chunk that has speech.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **A recording without speech gives a failure, not text.** A chunk is left out when the model's odds of "no speech"
  pass 0.6, whatever its text would have been; with no chunk left, the outcome is "No speech was found in the audio
  file." [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
- **The transcript can be edited** in its text field before it is copied or exported.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **A .txt export is the transcript; a .md export is the audio file's name as a heading, then the transcript.**
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **Three ways in:** the Open dialog, a file dropped on the window, and a file named as the first argument.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)

## Code and files

- **The window and the worker share nothing.** Jobs go to the worker and events come back, over channels; there is never
  more than one job.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **One list of models,** `src/catalog.rs`, read by the build script and the window alike.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **The models are downloaded by the build script,** with curl, into `models/`, which is not in Git.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **Only a release build holds the models.** Any other build reads them from `models/`.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **The C runtime is built into the executable,** so nothing is installed beside it.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **`opus-decoder` is built without overflow checks,** until the crate mends its shift in `celt/vq.rs`.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **A crash inside a transcription is a failure,** shown in the window, not a window left waiting.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **Chunks are cut at pauses,** the quietest tenth of a second in the last five of each 30.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **The likeliest token is always taken,** with no times and no second tries.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **File dialogs are behind `Dialogs`,** so that a test can answer in their place.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **One dark theme, with text at 7:1 or more** against its background.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)

## Tests

- **All tests are in `tests/suite.rs`,** each named by its number, so that they run in the order of `TESTS.md`.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **The fixtures are a computer voice,** Windows' own, so no person's voice is in the project.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **The tests transcribe with the tiny model,** and get it from `embedded`, as the executable does.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
- **Accessibility has no tests,** since Rule 5 names no purpose they would serve. Replaced: Tests 16 and 17, for
  `.github/ACCESSIBILITY.md`.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
- **crapper, dryer and mutator are run as `MAINTAINER.md` says,** mutator with one worker.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)

## Documents

- **The project is set up as speech-to-text,** with the owner and the chief developer read from Git.
  [2026-11-16 17:10:24](DEVLOG/DEVLOG1.md#171024-the-project-set-up-from-the-template)
- **The most common maintenance task is building the executable;** it is the first chapter of `MAINTAINER.md`.
  [2026-11-16 18:25:39](DEVLOG/DEVLOG1.md#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported)

- **Security holes are reported at nordenbrink.fi/contact,** as `.github/SECURITY.md` says.
  [2026-11-16 20:16:17](DEVLOG/DEVLOG1.md#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left)
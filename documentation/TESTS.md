# Tests

This project strives to have sensible testing. The goal is not to have 100% test coverage but to have tests which _sufficiently_ ensure the functional completeness, functional correctness, maintainability, and accessibility of speech-to-text. For more guidelines on software quality and accessibility, see [ISO/IEC 25010:2011](https://cdn.standards.iteh.ai/samples/35733/2ca18b477b7845a5b8cae39d6de0c098/ISO-IEC-25010-2011.pdf), [ISO/IEC 25010:2023](https://cdn.standards.iteh.ai/samples/78176/13ff8ea97048443f99318920757df124/ISO-IEC-25010-2023.pdf) and [WCAG 2.2](https://www.w3.org/TR/WCAG22/).

This document lists each test, its purpose, and in which test file it resides. This document is an effort to mitigate redundant tests and ensure test quality. It is intended as an aid in the trouble-shooting of potential false positives.

Test descriptions must remain brief and parameters (eg. Purpose) may not exceed 120 characters. Tests are ordered in order of their execution.

The test report names each test by its number here, such as "Test 15 failed": a test's number is its place in the order
the tests run. So when a test is added, removed or moved, renumber this document to match.

Each test tests one thing, for a requirement, for a rule, or to catch again a bug found before (Rule 5).

<!--
Each test has an entry of this form, after the table of contents, in the order the tests run:

## Test N

### File residence:
`path/to/test_file`, lines 24-26

### Purpose:
Verify that ... (Req. 3, 4).

### Assertion expectation:
What must hold for the test to pass.
-->

## Table of contents per category

Functional completeness & correctness tests:

- [Test 1](#test-1): an .ogg file with Vorbis sound is transcribed
- [Test 2](#test-2): an .ogg voice note with Opus sound is transcribed
- [Test 3](#test-3): no overflow on Opus sound in CELT mode
- [Test 4](#test-4): a .wav file is transcribed
- [Test 5](#test-5): a stereo recording is transcribed
- [Test 6](#test-6): a recording over 30 seconds is transcribed whole
- [Test 7](#test-7): no text is made up for a recording without speech
- [Test 8](#test-8): the transcript is shown in a text field
- [Test 9](#test-9): Copy copies the transcript
- [Test 10](#test-10): export as .txt
- [Test 11](#test-11): export as .md
- [Test 12](#test-12): the model chosen in the window goes with the job
- [Test 13](#test-13): each job is transcribed by its own model
- [Test 14](#test-14): a file dropped on the window is transcribed
- [Test 15](#test-15): a file dropped during a transcription starts no job
- [Test 16](#test-16): the command line writes the transcript to standard output
- [Test 17](#test-17): the model named on the command line goes with the job
- [Test 18](#test-18): the command line saves the transcript to the file named by --output
- [Test 19](#test-19): a failure ends the command line with an error status
- [Test 20](#test-20): an audio file named as the only argument opens the window

Accessibility tests:

None.

Maintainability tests:

- [Test 21](#test-21): no comments in the code
- [Test 22](#test-22): no function over 60 lines

## Test 1

### File residence:
`tests/suite.rs`, lines 172-175

### Purpose:
Verify that an .ogg file holding Vorbis sound, at 44 100 Hz, is transcribed to text (Req. 1, 2).

### Assertion expectation:
The transcript of `fox_vorbis.ogg` by the tiny model, in lower case, contains "quick brown fox".

## Test 2

### File residence:
`tests/suite.rs`, lines 177-180

### Purpose:
Verify that an .ogg voice note, Opus sound in SILK mode, is transcribed to text (Req. 1, 2).

### Assertion expectation:
The transcript of `fox_opus_silk.ogg` by the tiny model, in lower case, contains "quick brown fox".

## Test 3

### File residence:
`tests/suite.rs`, lines 182-185

### Purpose:
Prevent the Opus decoder's shift overflow from stopping .ogg files in CELT mode again (regression).

### Assertion expectation:
`fox_opus_celt.ogg` is transcribed without a panic, and the transcript contains "quick brown fox".

## Test 4

### File residence:
`tests/suite.rs`, lines 187-190

### Purpose:
Verify that an audio file that is not .ogg, a .wav, is transcribed to text (Req. 1, 2).

### Assertion expectation:
The transcript of `fox.wav` by the tiny model, in lower case, contains "quick brown fox".

## Test 5

### File residence:
`tests/suite.rs`, lines 192-195

### Purpose:
Verify that a recording with two channels, here Opus sound, is transcribed to text (Req. 1, 2).

### Assertion expectation:
The transcript of `fox_opus_stereo.ogg` by the tiny model, in lower case, contains "quick brown fox".

## Test 6

### File residence:
`tests/suite.rs`, lines 197-200

### Purpose:
Verify that a recording longer than one 30-second chunk is transcribed whole (Req. 2).

### Assertion expectation:
The transcript of the 34-second `long_vorbis.ogg`, in lower case, equals `LONG`, all eight sentences.

## Test 7

### File residence:
`tests/suite.rs`, lines 202-206

### Purpose:
Prevent text from being made up for a recording without speech, as "you" was for silence (regression).

### Assertion expectation:
The only outcome for `silence.ogg` is the failure "No speech was found in the audio file."

## Test 8

### File residence:
`tests/suite.rs`, lines 208-214

### Purpose:
Verify that the transcript is shown in a text field (Req. 3).

### Assertion expectation:
After the event that ends a job, the value of the window's multiline text input equals the transcript sent.

## Test 9

### File residence:
`tests/suite.rs`, lines 216-224

### Purpose:
Verify that the user can copy the transcript from the window (Req. 3).

### Assertion expectation:
Pressing "Copy" makes the window ask for the whole transcript to be put on the clipboard.

## Test 10

### File residence:
`tests/suite.rs`, lines 226-230

### Purpose:
Verify that the transcript is exported as .txt (Req. 4).

### Assertion expectation:
After "Export as .txt", the chosen file holds the transcript followed by one line break.

## Test 11

### File residence:
`tests/suite.rs`, lines 232-236

### Purpose:
Verify that the transcript is exported as .md (Req. 4).

### Assertion expectation:
After "Export as .md", the file holds "# voice note", an empty line, the transcript and a line break.

## Test 12

### File residence:
`tests/suite.rs`, lines 238-244

### Purpose:
Verify that the model chosen in the window is the one a job is given (Req. 7).

### Assertion expectation:
After "base" is chosen from Model, the job sent when a file is opened has `Model::Base`.

## Test 13

### File residence:
`tests/suite.rs`, lines 246-250

### Purpose:
Verify that the worker transcribes each job with the model the job names (Req. 7).

### Assertion expectation:
For two jobs, tiny then base, the worker asks for the files of exactly `[Tiny, Base]`, in that order.

## Test 14

### File residence:
`tests/suite.rs`, lines 252-257

### Purpose:
Verify that an audio file dropped on the window is taken as input (Req. 1).

### Assertion expectation:
After `dropped.ogg` is dropped on the window, the job sent names that file.

## Test 15

### File residence:
`tests/suite.rs`, lines 259-265

### Purpose:
Prevent a file dropped during a transcription from starting a second job again (regression).

### Assertion expectation:
After two files are dropped one after the other, with no job ended between them, one job has been sent.

## Test 16

### File residence:
`tests/suite.rs`, lines 267-271

### Purpose:
Verify that the command line writes the transcript of an audio file to standard output (proposed Req. 8).

### Assertion expectation:
`transcribe fox.wav --model tiny` run by the executable prints, in lower case, text containing "quick brown fox".

## Test 17

### File residence:
`tests/suite.rs`, lines 273-281

### Purpose:
Verify that the model named on the command line is the one the job is given (proposed Req. 7).

### Assertion expectation:
`transcribe "voice note.ogg" --model base` is read as a transcription whose job has `Model::Base`.

## Test 18

### File residence:
`tests/suite.rs`, lines 283-291

### Purpose:
Verify that the command line exports the transcript as .md to the file named by --output (Req. 4, proposed Req. 8).

### Assertion expectation:
After `transcribe fox.wav --format md --output <file>`, the file starts "# fox", an empty line, "The quick brown fox".

## Test 19

### File residence:
`tests/suite.rs`, lines 293-296

### Purpose:
Verify that a failure ends the command line with an error status (proposed Req. 8).

### Assertion expectation:
`transcribe silence.ogg --model tiny` run by the executable ends with exit status 1.

## Test 20

### File residence:
`tests/suite.rs`, lines 298-305

### Purpose:
Verify that an audio file named as the only argument, as "Open with" names it, is taken as input (Req. 1).

### Assertion expectation:
`cli::parse` of the one argument `voice note.ogg` gives a window that starts with `voice note.ogg`.

## Test 21

### File residence:
`tests/suite.rs`, lines 307-316

### Purpose:
Enforce that the code has no comments (Rule 10).

### Assertion expectation:
No `.rs` file in `src/`, `prebuild/` or `tests/` holds a line or block comment; the failure lists the files.

## Test 22

### File residence:
`tests/suite.rs`, lines 318-322

### Purpose:
Enforce that no function exceeds 60 lines of code (Rule 11).

### Assertion expectation:
The longest function in the `.rs` files of `src/`, `prebuild/` and `tests/` has at most 60 lines, blank ones aside.

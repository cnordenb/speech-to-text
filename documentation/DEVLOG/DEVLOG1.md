# Development log, part 1

Part 1 of the development log. The decisions made in the log are listed in [DECISIONS.md](../DECISIONS.md).

- [2026-11-16](#2026-11-16)
- [2026-11-23](#2026-11-23)

## 2026-11-16

### 17:10:24: the project set up from the template

#### What changed

- **Why.** The repository was a fresh copy of the template, and `check-rules.sh` fails on a project that is not set
  up. The setup was approved together with the request for the application, under the name of the repository.
- **What the project is for.** speech-to-text is a Windows application that transcribes audio files, such as .ogg, to
  text that can be copied or exported as .md or .txt.
- **Placeholders.** `PROJECT_NAME`, `PROJECT_DESCRIPTION`, `GITHUB_OWNER`, `REPO_NAME`,
  `CHIEF_DEVELOPER` and `AI_MODEL` were filled in wherever they stood: `README.md`, `AGENTS.md`,
  `RULES.md` (Rules 1 and 2), `MAINTAINER.md`, `TESTS.md`, and the two forms in `.github/ISSUE_TEMPLATE/`. The
  owner and the name of the repository were read from the address of `origin`, and the chief developer from Git's
  user name.
- **README.md.** The note that says the repository is a template was removed.
- **LICENSE.** Left as it was: it already carried this year and a holder.
- **Checked.** `bash .github/scripts/check-rules.sh` passed and listed the placeholders left under `todo`.
- **Documents.** `DECISIONS.md`.

#### Not done

- `REQUIREMENTS.md` is still the template's line 0. It is the chief developer's to write (Rule 2).
- Placeholders left at this point: `STACK`, `TEST_COMMAND`, `LANGUAGE`, `USAGE`,
  `MAINTENANCE_TASK`, `MAINTENANCE_TASK_ANCHOR`, `MAINTENANCE_STEPS`, `CODE_OVERVIEW` and
  `SECURITY_CONTACT`.
- `documentation/RULES.md` held a change to Rule 5 that was not committed when the work began. It was left as found.

#### Questions

1. **The chief developer's name in Rule 2.** Rule 2 now reads "cnordenb @cnordenb", because the setup takes the name
   from Git's user name. `LICENSE` names its holder in full. Should Rule 2 carry the full name instead? Proposal: yes,
   changed by the chief developer, since `RULES.md` is not changed without being asked.

### 18:25:39: audio files transcribed to text that can be copied and exported

#### What changed

- **Why.** An application was requested that takes audio files, .ogg among them, and transcribes their speech to
  text; the text is to stand in a text field it can be copied from, and be exportable as .md or .txt; all in Rust;
  ending in a finished Windows executable. Four things were asked before the work began, and answered: Rust may be
  installed, the project is set up from the template, the model is chosen in the application, and the models are
  built into the executable.
- **Rust.** Rust 1.100 was installed with rustup, and later `cargo-llvm-cov` with the `llvm-tools-preview` component,
  which crapper needs for Rust.
- **Audio files.** `decode` in `src/audio.rs` reads .ogg, .wav, .mp3, .flac and .m4a with symphonia, averages the
  channels into one and brings the samples to 16 000 a second with rubato. An Opus track, which is what a voice note
  holds, is decoded by `opus-decoder`, because symphonia has no Opus decoder.
- **Transcription.** `Transcriber` in `src/whisper.rs` runs OpenAI's Whisper models with candle. The samples are cut
  into chunks of at most 30 seconds, each cut at the quietest tenth of a second in the last five. A chunk the model
  takes for speechless is left out. The language is read from the first chunk with speech. Each token is the
  likeliest one, and no times are written.
- **The window.** `App` in `src/app.rs`, drawn with eframe and egui: "Open audio file…", a Model list (tiny, base
  and small, with small chosen at the start), a status line with a progress bar, the Transcript text field, which can
  be edited, and the buttons "Copy", "Export as .txt" and "Export as .md". A file can also be dropped on the window,
  or named as the program's first argument, as Explorer's "Open with" does. The theme is dark.
- **The worker.** `serve` in `src/app.rs` runs on a thread of its own and shares nothing with the window: jobs go one
  way and events the other. It keeps the model it loaded last. A crash inside a transcription becomes a failure.
- **Export.** `src/export.rs`: a .txt file holds the transcript; a .md file holds the audio file's name as a heading,
  then the transcript.
- **The models.** `src/catalog.rs` is the one list of them. The build script, `prebuild/embed.rs`, downloads their
  files with curl into `models/`, which Git leaves out, and writes the function `embedded` that `src/lib.rs`
  includes. A release build holds the files themselves; any other build reads them from `models/`.
- **The executable.** `target/release/speech-to-text.exe`, 1.43 GB, with Microsoft's C runtime built in
  (`.cargo/config.toml`), so that nothing has to be installed beside it.
- **Placeholders.** `STACK`, `TEST_COMMAND`, `LANGUAGE`, `USAGE`, `MAINTENANCE_TASK`, `MAINTENANCE_TASK_ANCHOR`,
  `MAINTENANCE_STEPS` and `CODE_OVERVIEW` were filled in. `SECURITY_CONTACT` is the only one left.
- **Checks on GitHub.** `check-rules.sh` now measures `.rs` and `.toml` files as code. `checks.yml` installs Rust and
  runs `cargo test`.
- **Tests.** 19, all new, in `tests/suite.rs`. Tests 1 to 7 transcribe fixtures with the tiny model: Vorbis at
  44 100 Hz, an Opus voice note, Opus in CELT mode, .wav, stereo, 34 seconds, and silence. Tests 8 to 15 work the
  window and the worker: the text field, Copy, the two exports, the model, and dropped files. Tests 16 and 17 are for
  accessibility. Tests 18 and 19 enforce Rules 10 and 11.
- **Checked.** All 19 tests passed. The release executable was started, with the small model, on the voice note, the
  stereo fixture, the 34-second fixture and the silent one: three transcripts right word for word, the 34 seconds in
  about 25, and "No speech was found in the audio file." for the last. Five tests were seen to fail first: Test 3 on
  the overflow, Test 7 with "you" for silence, Test 15 with two jobs sent, Test 17 at a contrast of 4.3, and Test 13
  when the worker's comparison of models was turned round on purpose, which was undone. crapper: no function over
  8.0. dryer: no duplicate. mutator: 38 of 54 mutants caught, 15 not, 1 on a line no test runs.
  `bash .github/scripts/check-rules.sh` passed.
- **Documents.** `TESTS.md`: Tests 1 to 19. `MAINTAINER.md`: written in full, "Build the executable" its first
  chapter. `TERMINOLOGY.md`: audio file, transcription, transcript, model, samples, chunk, export, failure, job,
  worker, event, fixture. `README.md`: Usage and the language badge. `AGENTS.md`: the stack, the test command, and
  that the three tools were tried in Rust. `DECISIONS.md`.

#### Obstacles met, and how they were solved

- **Opus in CELT mode stopped the tests.** `opus-decoder` 0.1.1 shifts a number too far in `celt/vq.rs`. A release
  build lets that pass; a test build stopped with "attempt to shift left with overflow". The crate is now built
  without overflow checks in test builds too (`Cargo.toml`), and Test 3 keeps watch.
- **Text was made up for silence.** With the rule of Whisper's reference code, six seconds of silence gave "you", and
  noise gave one phrase repeated to the limit. The model's own odds of "no speech" were measured: about 0.002 for
  the spoken fixtures, 0.85 to 0.94 for silence and hum. A chunk is now left out when those odds pass 0.6, whatever
  its text would have been, and Test 7 keeps watch.
- **A file dropped during a transcription started a second job**, and the first job's transcript then stood under
  the second file's name. A dropped file is now ignored while a job is under way, and Test 15 keeps watch.
- **The models in every build.** Building 1.4 GB into each test build took half a minute more a time, and the build
  folder grew to 13 GB. Only release builds hold the models now.
- **crapper showed 0 % for every function.** `cargo llvm-cov`, run the usual way with a wrapper round the compiler,
  measured only the build script under this version of Cargo. With `--no-rustc-wrapper` it measures all the code;
  crapper is then run with `--use-existing-coverage`.
- **mutator caught every mutant, then too few.** Its workers leave out a folder named `build`, where the build script
  then lay, so nothing compiled in them and every mutant counted as caught; the folder became `prebuild/`. Then four
  workers sharing one build folder overwrote one another's builds; one worker is used now. `MAINTAINER.md` has the
  commands.
- **What mutator's survivors led to.** Removed, as code no test could tell from its absence: a second name tried for
  the "no speech" token, a percentage beside the progress bar, and two switches for forgetting the model's memory,
  now one call in `encode`. Tests now use `embedded`, the function the executable uses, not a copy of their own.
  Added, because the behaviour is one a user meets: Test 5 (stereo), Test 13 (each job its own model), the Vorbis
  fixture at 44 100 Hz, without which leaving out the change of sample rate went unseen, and the whole transcript
  in Test 6.

#### How the rules were read

- **Rule 1.** `REQUIREMENTS.md` is empty and may not be written here (Rule 2). The request was taken as the
  requirements, in the seven points proposed under Questions, and `TESTS.md` cites them as "proposed Req.".
- **Rules 5 and 25.** Rule 5 allows a test for a rule; Rule 25 names only requirements and bugs. Tests 18 and 19, for
  Rules 10 and 11, were written, as the procedure for `TESTS.md` asks. Tests 16 and 17 serve
  `.github/ACCESSIBILITY.md`, which is neither a rule nor a requirement.
- **Rules 12, 13 and 26.** `.rs` and `.toml` files are code. `Cargo.lock` was read as not code: Cargo writes it, as
  npm writes the `.json` lock files that Rule 26 leaves out, and it has thousands of lines.
- **Rule 14.** With `Cargo.toml` and `Cargo.lock` the main folder holds its eight files, so the build script has a
  folder of its own.
- **Rule 15.** One package, so one `Cargo.toml`, one `lib.rs` and one `main.rs`.
- **Rule 25 and ub-mode.** No test was written for crapper's or mutator's sake. The survivors that only such a test
  would catch are listed under Not done.

#### Not done

- `REQUIREMENTS.md` is still to be written by the chief developer.
- The checks on GitHub have not run, since nothing was pushed. Whether its Linux machine builds the window's
  libraries, and has the time and room for 1.4 GB of models, is not known.
- .mp3, .flac and .m4a were not tried, and have no test.
- Only English spoken by a computer voice was tried: no person's voice, no other language.
- Not tried by hand: the two file dialogs, the Windows clipboard after "Copy", a file dragged from Explorer, and
  "Open with". The tests put their own answers in place of the dialogs and of the drop.
- The executable was not tried on another computer. It is not signed, so Windows may warn at its first start. It has
  no icon and no installer.
- No test makes a failure happen other than "no speech": a file that is not audio, or cannot be read or written.
- mutator's 15 survivors. Nine change nothing that can be seen: `>` for `>=` at the 30-second and the 0.6 limits, a
  colour moved by one step, two memory switches that only cost time, the language range widened by a token that is
  never likeliest, special tokens that are never among the tokens decoded, and the Opus buffer halved while still
  large enough. Six are not pinned by any test: the fraction sent as progress, how loudness picks the pause to cut
  at, the limit of 224 tokens to a chunk (two), whether a build holds the models, and the mixed channels' volume.
  The pause in particular: the 34-second fixture gives the same transcript when cut at 25.0 seconds.
- `src/main.rs` and `run`, `Native` and `ui` in `src/app.rs` are run by no test; they were run by hand.
- Each token is the likeliest one. The reference code's second tries, with chance added, are not built, so noise can
  still give repeated text; the tiny model did so for eight seconds of white noise.
- The graphics card is not used. `medium` and `large` are not offered.
- `AGENTS.md`, `TESTS.md`, the procedure for `TESTS.md` and `check-rules.sh` still speak of 100 tests, which Rule 5
  no longer does. They were left as found.
- The faults met in `opus-decoder`, `cargo-llvm-cov` and mutator were worked round, not reported to their makers.

#### Questions

1. **The requirements.** Seven points are proposed for `REQUIREMENTS.md`, in the request's own order: (1) the
   application takes an audio file as input, and .ogg files are supported; (2) the speech in the audio file is
   transcribed to text; (3) the text is shown in a text field from which the user can copy it; (4) the user can
   export the text as a .md or a .txt file; (5) the application is written in Rust only; (6) the application is one
   executable for Windows; (7) the user chooses the model in the application, and the models are built into the
   executable. Are they right, and is anything missing? Once they are written, "proposed" comes out of `TESTS.md`.
2. **Three models in one executable.** The two answers were read together: tiny, base and small are built in, which
   makes 1.43 GB. `medium` is 3 GB more, and Windows runs no executable over 4 GB. Is that the wanted balance? The
   file could be halved by storing the models at half precision, or larger models fetched when first chosen.
   Proposal: keep it as it is.
3. **Tests for rules and for accessibility.** Should Tests 16 to 19 stay, though Rule 25 names only requirements and
   bugs? Proposal: keep them, and let Rule 25 name rules as Rule 5 does.
4. **The limit of 100 tests.** Rule 5 no longer has it; `AGENTS.md`, `TESTS.md`, the procedure for `TESTS.md` and
   `check-rules.sh` still do. Should they follow? Proposal: yes.
5. **Cargo.lock and Rule 26.** Is a lock file code? Proposal: add `*.lock` to Rule 26.
6. **Leaving out speechless chunks.** The rule now trusts the model's odds of "no speech" alone, unlike the
   reference code. A chunk of faint speech that the model takes for silence would be lost. Proposal: keep the rule,
   and look again if a recording loses speech.
7. **`SECURITY_CONTACT`.** Where should a security hole be reported? It is the last placeholder, in
   `.github/SECURITY.md`.

### 18:40:35: one mutator worker in MAINTAINER.md, and the decisions listed

#### What changed

- **Why.** The section of 18:25:39, [audio files transcribed to text that can be copied and
  exported](#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported), says that mutator is run with
  one worker, but the command in `MAINTAINER.md` still asked for four. Its decisions were not yet in `DECISIONS.md`.
- **Documents.** `MAINTAINER.md`, under Tests: the mutator command has `--max-workers 1`, and a point says why two or
  more workers count mutants wrongly. `DECISIONS.md`: the decisions of 18:25:39, and the one of 17:10:24, under the
  five subjects.
- **Checked.** All 19 tests passed again. `bash .github/scripts/check-rules.sh` passed. Every link in `DECISIONS.md`
  leads to a heading of `DEVLOG1.md`.

### 20:16:17: tests only for Rule 5's purposes, and no placeholder left

#### What changed

- **Answers.** Of the questions of 18:25:39, in [audio files transcribed to text that can be copied and
  exported](#182539-audio-files-transcribed-to-text-that-can-be-copied-and-exported): `REQUIREMENTS.md` was written
  with the seven points proposed (1); the three models, and the 1.43 GB they take, are kept (2); the limit of 100 tests
  is gone and Rule 5's purposes for a test take its place, with Rule 25 removed (3); every document that still spoke
  of the limit is to follow (4); `*.lock` joins Rule 24, which was Rule 26 (5); the rule for leaving out speechless
  chunks is kept (6); security holes are reported at nordenbrink.fi/contact (7). Of the question of 17:10:24, in
  [the project set up from the template](#171024-the-project-set-up-from-the-template): Rule 2 names the chief
  developer in full (1).
- **Rules.** Rule 25 was removed, and Rule 26 became Rule 24, before this section. In it, Rule 2 was given the full
  name that `LICENSE` holds, beside the name on GitHub, and Rule 24 was given `*.lock`.
- **Tests.** 17, down from 19. Tests 16, every control has a name, and 17, text contrast of at least 7:1, were
  removed: they served `.github/ACCESSIBILITY.md`, which is neither a requirement nor a rule, and Rule 5 names no
  other purpose for a test. Tests 18 and 19 became Tests 16 and 17, with nothing changed but their numbers. `contrast`
  in `tests/suite.rs` went with the old Test 17, and `visuals` in `src/app.rs` is no longer public, since only that
  test used it from outside.
- **The limit of 100 tests.** `TESTS.md` states Rule 5's purposes in its place. The procedures for `TESTS.md` and for
  checking the rules no longer speak of it. `check-rules.sh` no longer counts the tests, and names its check of
  `TESTS.md` after Rule 6 alone. `AGENTS.md` names Rule 5 alone where it named Rules 5 and 25; its own mention of the
  limit had already been taken out.
- **Placeholders.** `SECURITY_CONTACT`, in `.github/SECURITY.md`, is now a link to nordenbrink.fi/contact. With no
  placeholder left, the setup procedure, `.claude/skills/project-setup/`, and its row in `AGENTS.md` were deleted, as
  the procedure itself says; the procedure for checking the rules no longer points to it. `check-rules.sh` now prints
  "placeholders filled in".
- **Checked.** All 17 tests passed. Tests 16 and 17 were only renumbered, so they were not broken on purpose
  again. `bash .github/scripts/check-rules.sh` passed. crapper: no function over 8.05; `visuals` is now run by no
  test. dryer: no duplicate. mutator, on `src/app.rs` alone, since only the tests of that file changed: 4 of 5 mutants
  caught, none survived, and 1 on a line no test runs, a colour in `visuals`. The release executable was built again
  and, with the small model, transcribed the voice note word for word.
- **Documents.** `TESTS.md`: requirements cited by number, the old Tests 16 and 17 removed and 18 and 19 renumbered,
  "None." under accessibility, and Rule 5's purposes in place of the limit. `MAINTAINER.md`: 17 tests, and no `todo`
  or `skip` lines from `check-rules.sh`. `RULES.md`: Rules 2 and 24. `AGENTS.md`, `.github/SECURITY.md`, and
  `DECISIONS.md`.
- **Replaced.** Of 18:25:39: the request standing in for `REQUIREMENTS.md`, and the tests for accessibility.

#### How the rules were read

- **Question 3.** The answer was read as: a test needs one of the purposes Rule 5 names. Tests 18 and 19 enforce
  Rules 10 and 11, so they stayed; the two tests for accessibility serve neither a requirement nor a rule, so they
  went.
- **Rules and requirements, one by one.** Every rule and requirement was read against the project again. Each holds,
  or is listed under Not done. Requirement 5 holds on Windows: of all the dependencies, only two compile C code, and
  both are for Android and Linux.

#### Not done

- Nothing checks accessibility any more: neither the names of the controls nor the contrast of 7:1 that `visuals`
  still keeps (Question 1). `.github/ACCESSIBILITY.md`, the accessibility category of `TESTS.md`, and what the
  procedure for `TESTS.md` says of accessibility were left as found.
- No test shows Requirement 6, one Windows executable, or the half of Requirement 7 that has the models built in; the
  release build and the runs of the executable show them.
- mutator was run on `src/app.rs` alone, not on all the code.
- The rest of Not done of 18:25:39 still holds, but for `REQUIREMENTS.md` and the limit of 100 tests: among it, the
  checks on GitHub have not run, .mp3, .flac and .m4a were not tried, and neither were the dialogs, the clipboard, a
  file dragged from Explorer or "Open with" tried by hand.

#### Questions

1. **Accessibility.** `.github/ACCESSIBILITY.md` aims at WCAG 2.2 at level AAA, and the template has a category of
   tests for it, but with neither a requirement nor a rule behind it Rule 5 allows no such test. Nothing now checks
   that every control has a name a screen reader can say, or that text keeps a contrast of 7:1. Should accessibility
   become a requirement? Proposal: a requirement such as "The window meets WCAG 2.2 at level AAA", and the two tests
   written again.

## 2026-11-23

### 19:00:09: a command line in the same executable, on Windows and Linux

#### What changed

- **Why.** The project is to be offered as an agent skill on Agensi, a marketplace whose listings hold text only:
  an AI agent cannot use a window, so it must run the executable from a terminal, and agents also run on Linux. A
  command line, a Linux build confirmed to work, and a draft of the skill were asked for. Three choices were made
  before the work: the command line goes in the same executable; the draft of the skill stays outside Git; on Linux,
  both the command line and the window are tried.
- **The command line.** `src/app/cli.rs`, with `parse` and `start`. No arguments, or an audio file, opens the window
  as before. `transcribe <audio file>` writes the transcript to standard output, or with `--output <file>` exports
  it; `--model` takes tiny, base or small, small by default; `--format` takes txt or md, txt by default. `--help`
  and `--version` print the usage and the version. The exit status is 0 when the transcript was written, 1 on a
  failure, 2 on wrong usage. Progress is shown only in a terminal.
- **One worker for both.** A transcription on the command line runs `serve`, the window's worker, on a thread of its
  own. `serve` now takes any list of jobs, not only a channel. `Job::title` gives the name that both the window and
  the command line put in a .md export. `ExportFormat` has `ALL` and a default, `Text`, which the usage text lists.
- **A console program on Windows.** The executable was a window program at first, with the parent's console attached
  for the command line. PowerShell then neither waited for it nor passed on its output or exit status, so the
  executable is now a console program. `prebuild/embed.rs` writes a manifest into the Windows executable with
  `consoleAllocationPolicy` set to `detached`, so that Windows 11 24H2 and later open no console window when it is
  started from Explorer. On older Windows, `leave_own_console` lets go of a console that belongs to the window alone.
- **Version.** 0.2.0, up from 0.1.0, in `Cargo.toml` and `Cargo.lock`.
- **Checks on GitHub.** `checks.yml` runs the tests on Ubuntu and on Windows, and `check-rules.sh` on Ubuntu alone.
  The time limit is 60 minutes, up from 30.
- **The draft of the skill.** `temp/transcribe-audio/SKILL.md` and `LISTING.md`, outside Git, as Agensi lays a skill
  out: the description says when to use it and when not, a "Requires" line names the executable, and the agent is told
  never to download it, to write the transcript with `--output` and read that file, and what each exit status means.
- **Tests.** 21, up from 17. New: Test 16, the transcript on standard output; Test 17, the model named on the command
  line; Test 18, `--output` with `--format md`; Test 19, exit status 1 on a failure. Tests 16 and 17, for Rules 10 and
  11, became Tests 20 and 21, with nothing changed but their numbers.
- **Checked.**
  - All 21 tests passed on Windows, and on Ubuntu 26.04 (glibc 2.43) in WSL 2. Each new test failed when the code it
    guards was broken on purpose: no output, `--model` ignored, no export, exit status 0 on a failure. Each breakage
    was undone.
  - By hand on Windows: Git Bash, PowerShell and `cmd /c` wait for the command line, and get its output and exit
    status. The window started from Explorer has no console; started with a new console of its own, it lets the
    console go. Started with a fixture as its argument, it transcribes the fixture.
  - By hand on Linux, with the test build: the command line without a display gave the right transcripts, exit
    statuses 0, 1 and 2, and a .md export. The executable needs only glibc, libm and libgcc. Without a display, the
    window ends with exit status 1 and says why; under WSLg it showed the transcript of `fox_vorbis.ogg`.
  - The 34-second fixture took 5.2 seconds with tiny, 10.0 with base and 29.1 with small, with the release build on
    an 8-core Ryzen 7 3700X.
  - dryer: no duplicate. `bash .github/scripts/check-rules.sh` passed.
- **Documents.** `TESTS.md`: Tests 16 to 21. `MAINTAINER.md`: chapter 1 for Linux and for the memory a release build
  needs, the table of files, the diagram, "The command line", the dialogs on Linux, the build settings, the helper
  `command_line`, and the full folders. `TERMINOLOGY.md`: command line, agent skill, exit status; audio file and
  transcript widened. `README.md`: the description, "From the command line", the acknowledgements. `AGENTS.md`: the
  description. `DECISIONS.md`.

#### Obstacles met, and how they were solved

- **PowerShell did not wait.** See "A console program on Windows" above. The attaching of the parent's console was
  taken out with it.
- **A release build needs about 14 GB of memory.** rustc peaked at 14.2 GB, and failed with "rustc-LLVM ERROR: out of
  memory" when less was free; the small `src/main.rs` alone took 55 seconds and 14.2 GB. Putting the models' bytes in
  `static` items, and `#[inline(never)]` on `embedded`, each needed more memory, not less, and both were undone. The
  build is as it was before; `MAINTAINER.md` names the memory it needs.

#### How the rules were read

- **Requirement 6, "one Windows executable".** There is still one Windows executable; the Linux one is beside it, not
  instead of it, and the command line is in the same executable. The command line and Linux go beyond the
  requirements as they stand, so Tests 16 to 19 cite proposed requirements (Question 1).
- **Requirement 5, Rust only.** On Linux no dependency compiles C code: `cargo tree` for Linux holds no `cc`, and
  fontconfig and the window's libraries are loaded while the program runs. The Windows manifest is text that the Rust
  build script writes, as it writes `embedded`, and `GetConsoleProcessList` and `FreeConsole` are called from Rust.
- **Rule 14.** `src/` held its 7 files, so the command line is in a folder of its own, `src/app/`. `tests/fixtures/`
  is full too.

#### Not done

- No release build was made on Linux: the machine lacked the free memory for it. The Linux command line and window
  were tried with the test build, which reads the models from `models/` instead of holding them.
- On Linux without xdg-desktop-portal, as in WSL, "Open audio file…" and the export buttons do nothing, and nothing
  says why. Copy was not tried on Linux.
- Windows 10 and Windows 11 before 24H2 were not tried: whether a console window flashes there is not known.
- A person typing the command in cmd or PowerShell 5.1 sees letters outside ASCII printed in the console's code
  page; `--output` writes UTF-8. Not tried, as every fixture is English.
- crapper and mutator: see the next point of the log, if any; not run when this section was written.
- .mp3, .flac and .m4a are still untried. No release binaries exist for download, and the checks on GitHub have not
  run, since nothing was pushed.
- A failure such as a missing file reads "The system cannot find the file specified. (os error 2)", without the
  file's name, in the window as on the command line.

#### Questions

1. **Requirements 6 to 8.** Proposed: "6. It is one executable for Windows and one for Linux." "7. The model is
   chosen in the app or on the command line, and the models are built in." "8. It can be run from the command line:
   the transcript goes to standard output or is exported as .md or .txt, and a failure ends it with an error status."
   `TESTS.md` cites them as "proposed Req." until they are written.
2. **Memory for a release build.** About 14 GB, which GitHub's runners, with 16 GB, may not spare, and which many
   computers lack. Proposal: have the linker put the models into the executable as data, so that rustc never reads
   them.
3. **The Linux executable's age.** Built on Ubuntu 26.04, it needs glibc 2.43 or newer, so it does not run on older
   Linux. Proposal: build the released one on an older Ubuntu, or for musl.
4. **The skill on Agensi.** Listed free first, or for a price? The code is MIT and public, so a buyer pays only for
   `SKILL.md`. Proposal: free first, and four questions put to Agensi in writing before it is listed.

### 19:58:51: the command line's arguments read by smaller functions, and "Open with" tested

#### What changed

- **Why.** crapper scored three functions of `src/app/cli.rs` over 10: `parse` 21.8, `transcription` 14.1 and
  `start` 13.8. Before the command line, no function in the project scored over 8.05.
- **`transcription`.** Split in three: `transcription` takes every argument that does not start with `-` as the audio
  file; `set` reads one option and its value into the `Request`; `only` refuses none, or more than one, audio file.
  `value` is gone. One answer changed: when two audio files are named and an unknown option follows them, the
  option's error is now given instead of the files'. Both end with exit status 2.
- **`start`.** `Command::Help` and `Command::Version` became one `Command::Say`, the text to print, as both did the
  same. Each arm of `start` now calls one function: `window`, `transcribe`, `say`, or `wrong`, which prints the reason
  and the usage.
- **Tests.** 22, up from 21. New: Test 20, an audio file named as the only argument, as Explorer's "Open with" names
  it, opens the window (Req. 1). It was the one way in with no test, and `parse` now stands between it and the window.
  Tests 20 and 21, for Rules 10 and 11, became Tests 21 and 22, with nothing changed but their numbers.
- **Checked.**
  - Test 20 failed when `parse` was changed on purpose to drop the file, with `left: None`; the change was undone.
  - The answers of the command line to 13 kinds of wrong usage were recorded before the split, and were the same
    after it but for the one named above. `--help`, `--version` and an unknown argument were tried after the change
    of `start`.
  - All 22 tests passed with `cargo test`, and all 22 under `cargo llvm-cov`.
  - crapper, after the split and Test 20 but before the change of `start`: `transcription` 6.0, `parse` 8.5, `set`
    10.1, `start` 13.8, and no other function over 8.05. dryer: no duplicate. `bash .github/scripts/check-rules.sh`
    passed.
- **Documents.** `TESTS.md`: Test 20, and Tests 21 and 22. `MAINTAINER.md`: 22 tests in chapter 1, and how to add an
  option, with `set` and `only`. `DECISIONS.md`: the numbers of the tests of Rules 10 and 11, and Test 20.

#### Not done

- The coverage measurement after the change of `start` was stopped while its report was written, because the computer
  ran short of memory. So `start`'s new score is not known, and mutator was not run on `src/app/cli.rs`.
- `set` scores 10.1: its complexity is 10, five of it from `?`. The lines of it that no test runs are wrong usage,
  which no requirement names, so no test was written for them.
- One earlier run under `cargo llvm-cov` failed Test 19 and two other tests while memory was short. They passed alone
  under coverage, and all tests passed in the two full runs after it. The cause was not found.
- The changes of this section were not tried on Linux; the tests ran there before them.

### 20:04:06: the command line and the agent skill as they stand, and what is left

#### What changed

- **Why.** The work on the command line, on Linux and on the agent skill stops here for now. This section gathers in
  one place what is done, what is not, where the requirements and rules are strained, the open questions and the
  proposals, so that the work can go on from the log alone. The details are in the sections of
  [19:00:09](#190009-a-command-line-in-the-same-executable-on-windows-and-linux) and
  [19:58:51](#195851-the-command-lines-arguments-read-by-smaller-functions-and-open-with-tested).
- **Documents.** This section only. No code changed.

#### Where the work stands

- **The command line.** `speech-to-text transcribe <audio file> [--model tiny|base|small] [--format txt|md] [--output
  <file>]`, and `--help` and `--version`, in `src/app/cli.rs`. No arguments, or an audio file alone, opens the window.
  The exit status is 0 when the transcript was written, 1 on a failure, 2 on wrong usage. Version 0.2.0.
- **Windows.** The executable is a console program with a manifest asking for no console from Explorer, from Windows
  11 24H2 on. Git Bash, PowerShell and `cmd` wait for it and get its output and exit status. The window started from
  Explorer has no console, and started with a file it transcribes the file.
- **Linux.** Tried in WSL 2 with Ubuntu 26.04: Rust 1.100 installed there with rustup, the project tested in place
  from `/mnt/c/...` with `CARGO_TARGET_DIR=$HOME/stt-target`, so that the build stays out of the project's folder
  and `models/` is shared. The 21 tests of then passed. Without a display the command line gave the right
  transcripts and exit statuses; under WSLg the window showed the transcript. The executable needs glibc 2.43, for
  `atan2f` and other functions of the C library's maths, and otherwise only libm and libgcc.
- **Tests.** 22. All passed with `cargo test` on Windows, and under `cargo llvm-cov`.
- **Tools.** `bash .github/scripts/check-rules.sh` passed. dryer: no duplicate. crapper, last measured before the
  change of `start`: `set` 10.1, `start` 13.8, `parse` 8.5, `transcription` 6.0, every other function at most 8.05.
- **Speed.** For the 34-second fixture, with the release build on an 8-core Ryzen 7 3700X: tiny 5.2 seconds, base
  10.0, small 29.1.
- **The draft of the agent skill.** `temp/transcribe-audio/SKILL.md`, the skill, and `LISTING.md`, the text for the
  marketplace's form. `temp/skillisation.md` holds the marketplace's message. `temp/` is not in Git, so these exist
  only on the computer they were written on. The skill tells the agent to check `speech-to-text --version`, never to
  download the tool itself, to transcribe with `--output` to a file and read it, and what each exit status means.
- **Agensi, as found on its site.** A listing is a ZIP of `SKILL.md` and small text files; executables fail its
  security scan, and buyers get no download, so the skill can only name the tool under "Requires". The scan also fails
  a download piped into a shell, and network calls joined with reading local files. `name`, in lower case with
  hyphens and matching the folder, and `description` are required. Creators get 70 %; prices start at $1, card
  payment at $5; a refund within 30 days takes back the creator's share; listing is not exclusive. Its message gives
  150 000 visits and 20 000 users a month; its press release of July gave 50 000 and 3 000.
  Sources: https://www.agensi.io/docs/packaging, https://www.agensi.io/security, https://www.agensi.io/terms,
  https://www.agensi.io/docs/what-you-can-sell.

#### Not done

- **mutator** was not run on `src/app/cli.rs`, and `start`'s score after its change is not known: the coverage run
  was stopped for lack of memory, with other builds running on the same computer.
- **`set`** scores 10.1. Its untested lines are wrong usage, which no requirement names, so no test was written.
- **One coverage run** failed three tests while memory was short; the cause was not found.
- **Linux:** no release build, as it needs about 14 GB of free memory; Copy not tried; the dialogs need
  xdg-desktop-portal, which WSL lacks; the changes of 19:58:51 not tested there.
- **Windows 10, and Windows 11 before 24H2:** not tried; whether a console window flashes there is not known.
- **.mp3, .flac and .m4a:** not tried. `tests/fixtures/` is full (Rule 14), so new fixtures need a folder of their own.
- **GitHub:** the checks on Ubuntu and Windows have never run, as nothing has been pushed; no release binaries exist,
  so the address under "Requires" in the skill leads nowhere yet.
- **Messages:** a missing file reads "The system cannot find the file specified. (os error 2)", without the file's
  name. In `cmd` and PowerShell 5.1, printed letters outside ASCII follow the console's code page; `--output` writes
  UTF-8. Not tried.

#### How the rules were read

- **Requirement 6, "one Windows executable".** Read as still holding: there is one Windows executable, and the Linux
  one is beside it. Question 1 of 19:00:09 proposes wording that names both.
- **Requirement 7, "chosen in the app".** Holds; the command line chooses the model as well, beyond it.
- **Rule 5.** Tests 16 to 19 cite proposed Requirements 7 and 8, which are not yet written.
- **Rule 3.** A release build needs about 14 GB of free memory, which makes the most common maintenance task harder.
- **Rule 4.** The tests run on every push only once the checks on GitHub have run on both systems.

#### Next steps, in order

1. Answer the open questions below, and write Requirements 6 to 8; then "proposed" comes out of `TESTS.md`.
2. With about 15 GB of free memory and no other build running: coverage, crapper, and mutator on `src/app/cli.rs`,
   with the commands in `MAINTAINER.md`, under Tests.
3. In WSL: `cargo test` after the changes of 19:58:51, a release build, Copy, and the dialogs on a desktop that has
   xdg-desktop-portal.
4. Push a branch, so that the checks run on Ubuntu and Windows.
5. Try the window on Windows 10, or on Windows 11 before 24H2.
6. Release binaries, then the questions to Agensi, then the listing.

#### A stronger agent skill: proposals

1. **Binaries to download.** A workflow, started by a tag pushed by hand, that builds the Windows and Linux
   executables and attaches them to a GitHub release with their SHA-256 sums. GitHub takes files up to 2 GiB. The
   Linux one built on an older Ubuntu, or for musl, and the Windows one signed, as SmartScreen warns of an unsigned
   download of 1.4 GB.
2. **A smaller download.** 1.4 GB is the largest hurdle. Models stored at half precision take about half; a build
   with tiny and base alone is about 220 MB; models fetched when first chosen would need Requirement 7 changed.
3. **Offline and private first.** The marketplace mostly offers cloud services paid per call; "the audio never leaves
   the computer, and nothing is paid per minute" is the difference to lead with.
4. **Output for agents.** `--json` with text, language, model and duration; `--timestamps`; `--language` to set the
   language; `--translate` to English, which Whisper can do and `Tokens` already knows; several files in one call,
   with the model loaded once.
5. **Speed.** Agents' shells stop commands after 2 to 10 minutes, and small takes about 0.85 seconds a second of audio
   on a desktop processor. The graphics card, through candle's CUDA and Metal, would help.
6. **macOS on Apple Silicon,** where many agents run. It would need Requirement 6 widened again.
7. **Evidence of quality.** Only English in a computer voice is tested. Before a price: human voices, accents, a few
   languages, and .mp3, .flac and .m4a; and Whisper's retry at a higher temperature against text repeated on noise.
8. **A changelog** in `documentation/`, as the main folder is full, with the versions that the skill's "Requires"
   names.
9. **Four questions for Agensi, in writing, before the listing:** is a skill whose tool is downloaded apart from it
   accepted; is a free `SKILL.md` that is also on GitHub counted as repackaged; how does the scan treat instructions
   to run a local tool on the user's file; and does any of its 4 500 tools already transcribe speech.

#### Open questions from earlier sections

- [19:00:09](#190009-a-command-line-in-the-same-executable-on-windows-and-linux): the wording of Requirements 6 to 8
  (1), the memory for a release build (2), the Linux executable's glibc (3), and free or paid on Agensi (4).
- [2026-11-16 20:16:17](#201617-tests-only-for-rule-5s-purposes-and-no-placeholder-left): accessibility as a
  requirement (1).

#### Questions

1. **Where the skill's draft is kept.** `temp/` is not in Git, and the project is public under MIT. Proposal: a
   private repository for the skill, apart from the project, so that it is kept, and is not published before it is
   decided whether it is sold.
2. **The size of the models, again.** On 2026-11-16 the 1.43 GB of three models was kept. There is a new reason to
   look again: every user of the skill must download it. Proposal: store the models at half precision, if the
   transcripts of the fixtures stay the same.
3. **Signing the Windows executable.** A certificate costs money each year; without one, Windows warns before the
   download runs. Proposal: release unsigned first, and sign once the skill has users.

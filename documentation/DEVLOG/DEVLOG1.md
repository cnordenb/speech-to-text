# Development log, part 1

Part 1 of the development log. The decisions made in the log are listed in [DECISIONS.md](../DECISIONS.md).

- [2026-11-16](#2026-11-16)

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

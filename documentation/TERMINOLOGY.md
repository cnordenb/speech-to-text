requirement - a numbered statement in REQUIREMENTS.md of what the project must do, e.g. "Requirement 4"
rule - a numbered statement in RULES.md of how the project must be made and kept, e.g. "Rule 12"
chief developer - the person who alone writes the requirements, named in Rule 2
test - a check of the project, run on every push and listed by its number in TESTS.md, e.g. "Test 15"
log - the development log in documentation/DEVLOG/: what changed in the project, why, and what is left
part - one file of the log, DEVLOG1.md, DEVLOG2.md and so on, each of at most 1000 lines
day - the entry of the log for one date, titled by it, e.g. "2026-11-02", never split between two parts
section - one change recorded under its day, titled by its time if it was the first written in that minute
decision - a choice made in a section and listed by subject in DECISIONS.md, with a link to that section
question - a numbered point for the chief developer at the end of a section, open until a later section answers it
command line - using speech-to-text in a terminal, by a person, a script or an AI agent, e.g. "speech-to-text --help"
agent skill - a SKILL.md that tells an AI agent when and how to use the command line, e.g. one listed on Agensi
audio file - a file of recorded sound, opened, dropped on the window or named on the command line, e.g. "voice note.ogg"
transcription - turning the speech in an audio file into text
transcript - the text a transcription gives, shown in the text field to change and copy, or printed by the command line
model - one of the Whisper speech recognisers built into the executable: tiny, base or small
samples - the sound of an audio file as numbers, one channel at 16 000 a second, the form a model takes
chunk - a stretch of samples of at most 30 seconds, cut at a pause, that a model takes in one go
export - saving the transcript as a file, as plain text (.txt) or as Markdown (.md)
failure - a message telling the user why a transcription or an export could not be done
exit status - the number the command line ends with: 0 when the transcript was written, 1 on a failure, 2 on wrong usage
job - one audio file and the model chosen for its transcription
worker - the thread that carries out jobs one at a time, so that the window stays usable meanwhile
event - what the worker reports about a job: its progress, or its end with a transcript or a failure
fixture - a small audio file in tests/fixtures/ that a test has transcribed

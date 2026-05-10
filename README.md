[![progress-banner](https://backend.codecrafters.io/progress/shell/1985a4e9-de84-4875-8a73-b9519767bfc6)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

Notes

# The first stage

1. Flushing output buffer. If you don't do this no newline char is encountered so the $ is never printed. println!() does flush but moves to a new line.

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Terminal input has \n char so need .trim() to remove it.

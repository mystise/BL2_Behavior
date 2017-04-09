# Behavior

A BL2 modding utility. Converts from a log dump of a behavior provider to a tree of behaviors that's easier to read.

Note that it is rough around the edges and built to exactly what I needed at the time, it might not fulfill everyone's needs, but it should allow people to get started working on behaviors. You should definitely not use this as an example of good coding practice in Rust, I spent very little time architecting this and just built what worked.

## Compilation

    cargo build

This project was built for Rust version 1.17 Beta in April 2017. While this is the version used to build the project, earlier and later versions will likely work fine. (No promises)

To install Rust, go to https://www.rust-lang.org/en-US/

## Usage

    cargo run -- <options>

Run through cargo or the executable itself.

Command line options:

-f=\<file\> : input behavior log

The behavior tree is output to stdout, to output to a file use shell stdout redirection.

Example:

    cargo run -- -f=critical_ascension.txt > critical_output.txt
    
    behavior -f=critical_ascension.txt > critical_output.txt

## Syntax-ish

Won't have a full description, it's confusing and it definitely isn't 100% correct.

Events trigger the linked behaviors whenever they activate. A Behavior can have multiple linked Behaviors and Variables. Variable links make no sense as 90% of them don't even have a name, and yet somehow still work in game. A Behavior can be delayed by a certain number of seconds. The first index on the Behavior is its index in ConsolidatedOutputLinkData, the second index is its index in BehaviorData2.

## Examples

Given is the log dump of Critical Ascension's behavior provider and the corresponding output of this program.

## Support

This project is unsupported, use at your own risk.

## License

This project is licensed under the Apache V2, included in LICENSE-2.0.txt.

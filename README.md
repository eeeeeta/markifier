The Markifier
=============

[![Crates.io badge](https://img.shields.io/crates/v/markifier.svg)](https://crates.io/crates/markifier)

**The Markifier** is a small Rust script that generates pretty graphs and CSV data from a directory full of
appropriately-labeled pieces of work. Basically, it lets you turn a directory of marked work into a nice line
graph of your performance over time, with a line of best fit and the mean marked on for you.

## Wait, what?

This program basically just scratches a personal itch. I have a bunch of scanned pieces of work that are in
a directory something like this:

    amazing file 1 [90%].pdf
    crappy failure [45%].pdf
    recovered this time [100%].pdf

...and I thought it would be nice to graph the percentage values embedded in the work's filename.

## More details, please.

The program uses the following regex to analyse files in a directory and get a percentage value out of them:

    .*\[(?P<percent>.+)%.*\].*

Basically, it'll accept any file that has a percentage like this: `[90%]` somewhere in the filename. It's
lenient, and parses percentages as floats, so it'll parse `[33.3%]`, `[9001%]` and `[50% yay]` just fine.

This data is then ordered by last modified time, the mean calculated, and a CSV file outputted that conforms
to the following schema:

     <file index starting at 0>,<last modified in epoch time>,"<file title>",<percentage value>

So, the example directory above would produce data like:

     0,123232323,"amazing file 1 [90%].pdf",90
     1,123232333,"crappy failure [45%].pdf",45
     2,123232456,"recovered this time [100%].pdf",100

To be even nicer, **The Markifier** will then generate a `.gnuplot` script that [gnuplot](http://gnuplot.sourceforge.net/)
can use to plot your data. It even runs `gnuplot` for you and generates a `.png`, if you have it installed.

## Configuration

**The Markifier** takes a configuration file as the first argument when you run it on the command line, e.g:

    $ markifier config.toml

This conforms to the following format:

    [[subjects]]
    directory = "/path/to/my/documents/"
    results_path = "/path/to/my/results/file.csv"
    name = "Computer Science for Dummies"
    colour = "green"

## Installation

It's the moment you've all been waiting for, after reading this really long README...actually installing
**The Markifier**! Good news: **The Markifier** is hosted on crates.io, so simply:

    $ cargo install markifier

...and you're done!


## License
Licensed under the [Unlicense](http://unlicense.org/).

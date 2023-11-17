# Mini Vim

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

This program is an attempt to imitate [vim](https://github.com/vim/vim), which is one of the most popular lighweight code editors.

I have used rust language to create this project.

> This project is only for learning purposes and better alternatives already exist.

## Features

* Completely Command line
* Modal editor i.e. editing in different modes which allows for faster editing
* Built using rust - Fast and robust application

## Modal editing

Vim originally offers 4 modes for editing

1. Insert Mode
2. Command Mode
3. Visual Mode
4. Idle or Default

As of now, mini-vim supports 3 modes

Feature | Idle Mode | Insert Mode | Command Mode
---|---|---|---
How to begin | Default Mode | `i`, `o` or `a` | `:`
Navigation | `h`, `j`, `k`, `l`, `w` | N/A | N/A
Options | N/A | N/A | `q`:quit, `w`: write, `wq`: write and quit

## Deletion

Currently, two types of delete are supported
`x`: remove one character in idle mode
`dd`: remove one complete line

## External crates that I have used

* ratatui + crossterm: To create amazing Terminal User Interface
* clap: For easy and effective parsing of command line arguments
* anyhow: Easy error handling

## Improvementss needed

This project is at very initial stage. It was only meant for learning purposes
and vim or neovim already provide these capabilities in much more elegant manner.

Some major missing features are: -

* Visual mode
* Execution of command multiple times using numbers in command
* Finding and searching
* Macros
* Scope for personalization / plugins
* Syntax highlighting / linting
* Tabs
* Keeping files saved in a buffer

and many more which I might have missed but are a part of existing tools

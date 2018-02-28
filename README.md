# Slime
[![Build Status](https://travis-ci.org/jaroslaw-weber/slime.svg?branch=master)](https://travis-ci.org/jaroslaw-weber/slime)
[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/slime.svg
[crates.io]: https://crates.io/crates/slime

Handlebars based flexible static site generator library

## Why slime
Because it is slim(e) and flexible (like a slime...)

## What?
template + data = static page!

## How
- handlebars
- toml/json
- few simple helpers

## Why?
- more flexible design - can reuse/manipulate data by merging or modyfing loaded json
- modular - support for partial templates
- easy to learn - using already popular json and handlebars combination
- easy to start - copy example and edit json/handlebar files to create one static page without touching rust code

## Installation

Create new binary crate.
Run:
```
curl -s https://raw.githubusercontent.com/jaroslaw-weber/slime/master/init_slime.sh | bash
```
(setup folders and deploy scripts for github+travis)

## Show me

Folder structure:

```
- generated (generated html files)
- data (data to insert into templates)
 - index.toml
- templates
 - index.hbs (handlebar templates)
deploy.sh (deploying generated html files to github pages)
.travis.yml (travis build script)

```

Code:

```
extern crate slime;
use slime::Slime;

fn main() {
    //create new wrapper
    let s = Slime::default();

    s.initialize()

    //load data
    let data1 = s.load_json_data("data1").expect("failed to load json data");
    let data2 = s.load_toml_data("data2").expect("failed to load toml data");

    //generate html files
    s.generate("index", "json_version", &data1)
        .expect("failed to generate page with json data");
    s.generate("index", "toml_version", &data2)
        .expect("failed to generate page with toml data");
}

```

## So what is going on?

```
let mut s = Slime::default();
```
Create new wrapper.

```
s.initialize()
```
Load templates.

```
let data1 = s.load_json_data("data1").expect("failed to load json data");
let data2 = s.load_toml_data("data2").expect("failed to load toml data");
```
Load some toml/json data from data folder.


```
s.generate_html("index", "json_version", &data1)
    .expect("failed to generate page with json data");
s.generate_html("index", "toml_version", &data2)
    .expect("failed to generate page with toml data");
```
Generate html files.

# What is new?
Check out CHANGELOG.md file!

## So what next?
- [ ] slime binary mode (using slime library without writing rust code)
- [ ] improve api
- [ ] improve error messages (use failure crate)
- [ ] example how to create nice websites with css frameworks
- [ ] add tests
- [ ] write tutorial
- [ ] add links to projects using slime to README file
- [ ] add more examples
- [ ] improve installation script
- [ ] adding custom helpers for handlebars

## Projects using Slime
- https://github.com/jaroslaw-weber/cbt-diary
- https://github.com/jaroslaw-weber/jaroslaw-weber.github
- https://github.com/fishlang/fishlang

## Binary mode (not implemented)
Generating page without writing rust code.

Install slime with
```
cargo install slime
```
Initialize project with
```
cargo slime --init
```
Put data in data folder.
Put templates in template folder.
Push everything to github.
Setup travis and wait for build to finish.
Your website will be generated and deployed automatically!

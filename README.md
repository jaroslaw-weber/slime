# Slime
[![Build Status](https://travis-ci.org/jaroslaw-weber/slime.svg?branch=master)](https://travis-ci.org/jaroslaw-weber/slime)

handlebars & json based static site generator library.

## Why slime
Because it is slim(e) and flexible (like a slime...)

## What?
template + data = static page!

## How
- handlebars
- json
- few simple helpers

## Why?
- more flexible design - can reuse/manipulate data by merging or modyfing loaded json
- modular - support for partial templates
- easy to learn - using already popular json and handlebars combination
- easy to start - copy example and edit json/handlebar files to create one static page without touching rust code

## Show me

```
extern crate slime;
use slime::data::load_json;
use slime::html::generate;
use slime::template::load_all;

fn main() {
    let hb = load_all().expect("failed to get templates");
    let index_data = load_json("index.json").expect("failed to get index data");
    generate(&hb, "index", &index_data, "index").expect("failed to generate html");
}

```

## So what is going on?

```
let hb = load_all().expect("failed to get templates");
```
Load templates from "template" folder.


```
let index_data = load_json("index.json").expect("failed to get index data");
```
Load data from "data" folder.


```
generate(&hb, "index", &index_data, "index").expect("failed to generate html");
```
Smash the data into the template.

## So what next?
Maybe more high level api?


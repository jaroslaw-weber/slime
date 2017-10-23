# Slime
[![Build Status](https://travis-ci.org/jaroslaw-weber/slime.svg?branch=master)](https://travis-ci.org/jaroslaw-weber/slime)

Handlebars based flexible static site generator library

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

Folder structure:

```
- generated (generated html files)
- data (data to insert into templates)
 - index.json
- templates
 - index.hbs (handlebar templates)

```

Code:

```
extern crate slime;
#[macro_use]
extern crate serde_json;
use slime::Slime;
use slime::DataFormat;


fn main() {

    //create new Slime wrapper
    let mut s = Slime::new();

    //link "index.hbs" with "index.json"
    s.add_simple("index", DataFormat::Json).expect("failed to add page");

    let mut some_data = s.load_data("index", DataFormat::Json).expect("failed to load json data");

    //manipulate loaded data
    some_data["test"]=json!("changed");

    //link "index.hbs" with some manipulated data and generate "index2.html" (only save settings)
    s.add("index", &some_data, "index2");

    //try to generate a website
    s.run().expect("failed to generate website");
}

```

## So what is going on?

```
let mut s = Slime::new();
```
Creates new wrapper


```
s.add_simple("index", DataFormat::Json)
```
Links "index.hbs" with "index.json" and register new page inside wrapper


```
let mut some_data = s.load_data("index", DataFormat::Json)
```
Load some other data

```
s.add("index", &some_data, "index2");
```
Register "index.hbs" with passed data and sets output path to "index2.html"

```
s.run();
```
Generate website using registered data and paths.


## So what next?
- [ ] stabilize api
- [ ] allow toml format
- [ ] example how to create nice websites with bulma

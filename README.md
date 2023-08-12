# About

Porydelete is a CLI tool which allows you to delete specific parts of the 3.gen decomp projects such as Pokemon, Items, Maps, Tilesets, Events and more. This is a project inspired by the `strip_events.py` script on my [pret-decomp-resources](https://github.com/Voluptua/pret-decomp-resources) repository. 

I am working on this project because:
  - I am not good at coding with Rust and I want to get better using this language
  - I always wanted to create a romhacking Tool (even in the binary hacking times)
  - I am also working on my own romhack and I want to save storage by removing unused content in the final product such as unused Items or Pokemon


# State of development

There isnt a stable release yet but I managed to edit json files as I need for deleting map attributes. I can use that for more. Feel free to contribute

# Features

| Feature | Added? |
|---------|--------|
|Delete map events|✅| 
|Delete Maps|❌|
|Delete Tilesets|❌|
|List and Delete unused scripts|❌|
|Delete Unused (Unused code wrote by Game freak which isnt removed by default)|❌|
|Delete Pokemon|❌|
|Delete Items|❌|
|Delete Battle-Engine features (expansion only)|❌|

# Getting started

Follow these instructions to build this project: [INSTALL.md](https://github.com/Voluptua/Porydelete/blob/main/INSTALL.md)
Then place `porydelete` into the root of your project
Run `./porydelete --help` for more information

## To delete Map-Attributes run:

```./porydelete --ma <attributes>```

Available attributes to delete (can be placed in any order): 
  - `connections`
  - `object_events`
  - `warp_events`
  - `bg_events`
  - `coord_events`

Examples: \
  `./porydelete --ma connections coord_events`\
  `./porydelete --ma warp_events connections`


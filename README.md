# django-sanity

![](https://github.com/abhijat/django-sanity/workflows/Rust/badge.svg)

A collection of tools to do sanity checks on Django projects

#### Look for receivers for a model

```shell script
cargo run --bin receivers -- --source-roots <source-paths> --subject Person
```

#### Check if all of the variables in settings are in use somewhere

```shell script
cargo run --bin receivers -- --source-roots <source-paths> <venv-path> --subject Person
```

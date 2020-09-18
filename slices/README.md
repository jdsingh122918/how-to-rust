# Introduction

There is no concept of ownership in the slices. They let you reference contiguous sequence of elements in a collection rather than the whole collection. String literals are of the type `&str` which makes them `immutable`. The concept of `borrowing`, `references` along with `slices` ensure memory saftey in `Rust` thereby eliminating the need of `Garbage Collector`

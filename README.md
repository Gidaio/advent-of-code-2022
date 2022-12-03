# Advent of Code 2022 in Rust

Hey! Welcome to my solutions for Advent of Code 2022! A
[friend of mine](https://github.com/Ammonsh/AdventOfCode2022) decided to use this to learn Rust, so
I decided to do the same. I've dabbled in Rust before, so I'm not a 100% beginner, but a lot of the
finer details still escape me. Rust is weird, man, but I do like a lot about it. Anyways, I thought
this would also be a good place to record my thoughts on each day as I do them.

## Day 1

This one was pretty easy. I had to remind myself how file reading works, and
discovered/rediscovered the `lines` function on the `BufRead` trait. Using that and a simple
one-line-at-a-time approach meant this solution was pretty simple. I tried to use `Result`s and
stuff, but error handling confused me, so I just sidestepped it all and `unwrap`ped everything.

Part 2 was an easy variation on it. I was briely caught out by not sorting the array, and so I was
getting weird results. I would replace the first elf's stockpile that was lower than the currently
examined elf's stockpile, which meant zeroes remained Sorting the array before searching it meant
that it was always replacing the lowest value in the array. Hopefully that makes sense.

## Day 2

This time I made a more concerted effort to actually represent the problem in the codespace. Lots
of enums. I find it strange that enums can only be used by matching, and that they don't implement
the `Eq` trait by default. It makes them a little more friction-y to work with than other
languages' enums.

I was really tempted to represent the states as numbers, 0 for rock, 1 for paper, and 2 for
scissors. That would open the doors for some fun modular arithmetic for calculating wins and stuff.
I decided against it because it felt more "idiomatic" to represent the states as an enum and branch
off of that. I think it would be more efficient to do it numerically, though.

I also tried to make a greater effort of using `Results`. It's not great, and there were some areas
where I didn't quite know how to handle it, but it's there!

## Day 3

A Saturday! I had more time, so I really got to explore stuff. This problem was like, textbook set
territory, so I decided to see if Rust had any builtins for that. Otherwise I could just use a
vector. Lo and behold, the `HashSet`! Magic!

I initially approached this by just iterating over the bytes (since everything was ASCII, `u8`s
made more sense to me than `char`s) and adding them one at a time to the set, then looping over the
set and using the `contains` function. I did some research to see if `HashSet`s supported
intersections, and they did. It worked weird, though; `intersection` returns an `Intersection`,
which is not a set, but an `Iterator`? But then getting that iterator back into another set so I
could do the second intersection proved troublesome. I ended up just making all three sets, then
using `retain` to effectively filter out anything that wasn't in every set. It worked alright, and
though it didn't feel as idiomatic as a proper intersection, it was still pretty good.

I also discovered `from_iter` on `HashSet`, which was useful. Then I could just do

```rust
HashSet::from_iter(line.bytes());
```

rather than having to loop over the bytes and add them one at a time.

Finally, I was able to experiment a lot more with error handling and module structure. I created my
own error types and `Box`ed them up dynamically into a `Result` so I could also handle builtin `io`
errors. Then I realized I could use the `mod.rs` file in each folder to hold things that were
common to both solutions, which made a lot of sense for this one because there was more of that
than in the previous two days.

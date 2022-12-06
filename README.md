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

I also tried to make a greater effort of using `Result`s. It's not great, and there were some areas
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

## Day 4

The problem itself today was pretty simple. Parsing it was easy, and verifying it was easy, so I
won't comment too much on that. Mostly, I used today to experiment `impl` blocks on `struct`s. This
makes the actual called function super small. It essentially boils down to

```rust
Pair::new(&line).overlaps()
```

(I omitted some `?`s and stuff for legibility.)

One thing that is really cool about Rust is that you can have multiple `impl` blocks for the same
`struct`. In the `mod.rs` file for today, I included `Pair::new`, which just parses the line into a
`Pair` object. Because each part has a different objective, though, `part1.rs` and `part2.rs` both
have another `impl` block for `Pair`, adding `Pair.subset` and `Pair.overlaps`, respectively. Super
cool. Rust has a _lot_ of really neat features.

I also tried using an error `enum` today instead of just a bunch of structs. If it makes semantic
sense, it's a really good way to reduce boilerplate code. You only need to implement
`std::fmt::Display` and `std::error::Error` once for the whole enum, rather than once per error
type. Good stuff.

## Day 5

I decided early on that parsing out the initial state was not worth it. Because it's arranged
vertically and not horizontally, it's not really conducive to being read from a file. It would be a
nightmare of state and grossness, and so I though, "Hey. Nothing says I _have_ to read it from a
file." So I didn't.

My big new discovery today was `while let` loops. In day 3, I wrote a `loop` that called `.next` on
the iterator once, then decided to break if that was `None`. If it was `Some`, then there _should_
be two more lines, and I called `.next` twice more, instead erroring if they returned `None`. I
realized this could be done easier. The `for` loop should theoretically handle the first case, and
then I could just call `.next` twice more to get the extra lines. Unfortunately, that doesn't work.

I ended up finding
[this Reddit post](https://www.reddit.com/r/rust/comments/2pqcgt/while_let_someitem_iteratornext/)
that explains that `for` loops borrow the iterator for the entire duration of the loop, rather than
just while (internally) calling `.next`. Fortunately, it gave a good alternative: the `while let`
loop! Which is essentially exactly what I wanted, though not _quite_ identical. (Replies to the
post list at least one main difference.)

## Day 6

This one was fun! I usually look at these when they're released, but that's 9:00 PM my time, so
unless it's a weekend, I let my brain mull it over until lunch at work the next day. I eventually
settled on a sliding-window approach and it worked first try! I'm very happy with it. On my machine
it runs in "0" ms.

Basically, the way the algorithm works is by keeping track of the start and end points of the
window. When a new character is added to the window, the algorithm checks the new character against
every other character in the window for a duplicate. If a duplicate is found, the start position of
the window is updated to be the position immediately after the duplicate character. Because there
was a duplicate in that range, immediately after it is the soonest place the packet marker could
start. Once the window has expanded to size 4, we've found the packet marker.

Since it was a sliding window algorithm, doing part 2 was trivial: just increase the size of the
window from 4 to 14. Worked a charm.

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

## Day 7

Holy heck. This one was quite the rabbit hole.

I started by trying to build a tree where every node held a reference to its parent. It looked
something like:

```rust
struct Node<'node> {
    children: Vec<Node<'node>>,
    parent: Option<&'node Node<'node>>,
}
```

The idea was that every node would own its children, and have a reference to its parent. I ran into
trouble when updating things, though. Adding a new node to the array of children worked great, but
then modifying that child to point back to the parent couldn't be done because I'd already
transferred ownership of the child to the parent.

```rust
self.current_node.children.push(new_node);
new_node.parent = &self.current_node;
```

I tried instead setting the parent _through_ the parent again, but that didn't work. It's weirdly
circular and I'm not surprised it doesn't work, but I still don't understand _exactly_ why. My best
guess is that I'm borrowing the value twice, once on each side of the `=`? Or that it makes a
circular reference, and now Rust doesn't know how to handle it when it tries to clean it up.

```rust
self.current_node.children.push(new_node);
let inserted_index = self.current_node.children.len() - 1;
self.current_node.children[inserted_index].parent = &self.current_node;
```

My last try was basically reversing the first try, setting the parent first and then adding it to
the children, but that gave me a lifetimes error. So I was completely stumped.

It turns out
[this is basically just unsafe](https://stackoverflow.com/questions/28608823/how-to-model-complex-recursive-data-structures-graphs).
Instead, you should be using `Rc<>` and `RefCell<>`! So here's my best explanation as to how they
work.

`Rc<>` allows more than one thing to own a value. It does this by reference counting. `Rc::clone`
doesn't _actually_ clone the value, but it does increase the number of references to the value.
Then, when the newly created `Rc<>` goes out of scope, the `Drop` implementation reduces that count
by 1 and, if the number of things referencing it are now 0, frees the value itself. Why is this
helpful? Essentially, every node owns its children _and_ its parent. Rust doesn't have to untangle
the circular referencing anymore, because _when_ the value should be freed is figured out at
runtime.

The values in `Rc<>` aren't mutable, however. This is fine for a lot of things, but what if we do
need the value to be mutable? That's where `RefCell<>` comes in, Essentially, `RefCell<>` switches
from statically enforcing Rust's borrow rules at compile time to _dynamically_ enforcing them at
_runtime_.

If you've ever heard phrases like "turing completeness" and the "Halting Problem", you've probably
at least grazed the surface of one of the most weirdly fascinating things in computer science: some
things aren't just unknown, but have been proven to be unknowable. This also extends to mathematics
in general (check out Godel's Incompleteness Theorem), but the gist is that there are some things
that you cannot write a program for. One of those things is that the correctness of code cannot be
verified.

Well that's weird. Rust claims to do that! What gives? There are a couple of caveats to this
statement. The first is that the correctness of code cannot be verified for _turing complete_
languages. What does it mean to be turing complete? Essentially, it means that the language is
capable of calculating anything that can be calculated. Remember, not everything _can_ be
calculated or known, but for the things that can, a turing complete language can do it. There are
simpler languages (look up Chomsky's hierarchy) for which correctness can be verified, but they
cannot calculate anything calculable.

The second caveat is that the correctness of code cannot be verified _in the general case_. That
means there are no algorithms that can verify the correctness of an arbitrary piece of code. The
reality is, though, that most code isn't arbitrary. There are classes of things that we developers
do with code, and a lot of them are pretty well understood. This is why Rust works the way it does:
it's borrow checker rules are really good at verifying the correctness of the most commons pieces
of code. Those common pieces happen to just be a subset turing completeness, but that's fine. They
still do what we need them to.

So! Back to `RefCell<>`. Sometimes, what you need to do in code transcends the bounds of static
borrow checking. Doing borrow checking at runtime sort of extends the realm of what's possible. It
turns out most graph theory stuff lies outside the bounds of static borrow checking, but inside the
bounds of dynamic borrow checking. This is why `RefCell<>` ultimately solved my issue.

There are still things outside the realm of dynamic borrow checking, and for that, Rust provides
`unsafe`. I haven't done much with that (except some
[very poorly written OpenGL stuff](https://github.com/Gidaio/opengl-renderer-rust)), so I'm not
gonna talk about it.

## Day 8

This was stark contrast to yesterday. It was very, very straightforward. I get the feeling that
there's a more optimal way to do these problems, but I couldn't think of one and was already
running a little behind. Could I have abstracted this code more? Sure. Sometimes, though, it's nice
to just do it the straightforward way.

I did take the opportunity to explore iterators a little bit more. Specifically, I learned the
difference between `.iter`, `.iter_mut`, and `.into_iter`.

`.iter` takes a reference to a collection and returns an iterator over references to the items in
that collection.

```rust
let item: Option<&Tree> = Vec::<Tree>::new().iter().next();
```

`.iter_mut`, predictably, takes a mutable reference to a collection and returns an iterator over
mutable references to the items in that collection.

```rust
let item: Option<&mut Tree> = Vec::<Tree>::new().iter_mut().next();
```

`.into_iter` is a little different. It actually takes ownership of a collection and returns an
iterator straight-up over the items in that collection. Once you use this method, you can't use the
original collection again; the iterator now has ownership of the values. Supposedly, this is what
`for ... in` loops "call" internally, but I haven't experimentally confirmed this.

```rust
let item: Option<Tree> = Vec::<Tree>::new().into_iter().next();
```

## Day 9

I really enjoyed this one! I started just super straightforward: a `Rope` struct with a `head` and
a `tail`. I made a `Point` struct that wrapped up an (x, y) pair to be easier to work with, and
then just did some basic if statements to work out part 1. Super simple.

Then I looked at part 2 and realized it was basically asking for a "discrete" forward kinematics
simulation. So instead of a `Rope` being modeled as a head and a tail, it was now modeled as a
`position` and then the `next` segment of the rope, which was just an `Option<Box<Rope>>`. (This
didn't need to be a `Rc<RefCell<Rope>>` because segments didn't need a reference to their parent.
Ownership was strictly one-way.)

Each segment would move, and then if it's child was too far away, instruct its child to move. This
was repeated recursively, turtles-all-the-way-down style. Because I was passing directions around,
and those directions could be diagonal, I decided to expand my `Point` struct into a partial
`Vector2` implementation. (It only has `Sub` and `AddAssign` traits.)

I initially modeled the logic as a series of 9 `else if` statements, one for each of the nine
regions the child segment could be in. Eventually, I realized that as long as the child segment was
at least 2 units away in either direction, then the child moved the distance "clamped" to one. So
if the parent was `(2, -1)` away from the child, the child moved `(1, -1)`. But if the parent was
`(0, 2)` away, the child only moved `(0, 1)`, and if the parent was `(-1, 0)` away, then the child
didn't move. Does that make sense? I dunno. It's very elegant in the code, though.

Each segment kept track of the positions it'd been. (I could've had each segment have an
`Option<HashSet>`, but I didn't want to track all of that.) At the end, to get the tail's
positions, I just dug down recursively until I found the tail segment (the one that didn't have a
child) and returned it back up.

To sum up, this challenge was fun not because the solution was immediate, but because the code sort
of "crunched itself down" over time into something that I'm really proud of.

## Day 10

This one wasn't too hard. I've [done some VM stuff before](https://github.com/Gidaio/lox-rs), so I
kind of knew my way around. The interesting thing I explored today was traits! Since each part
needed the core CPU, but did different things with the register and clock count, I decided to make
a `Peripheral` trait. This included a `::new` function so the CPU could create it, and an `.update`
method that was called each clock cycle. The cool part is when defining CPU, it's defined as a
generic that satisfies a trait:

```rust
struct CPU<T: Peripheral> { ... }
```

There was an actual type being passed in, so I didn't have to initialize the peripheral outside of
the CPU and pass it in, I could have the CPU initialize the peripheral itself! Super cool stuff.

## Day 11

I wasn't as big a fan of this one. Most of it just kind of felt like pointless busywork? There were
two things I liked about this challenge, though.

The first was the expression parsing. Each monkey had an "Operation", like `new = old * 4` or
`new = old + old`. Parsing that out and executing it was pretty interesting, and involved a lot of
the skills I learned/am learning in [Crafting Interpreters](http://craftinginterpreters.com/). I
basically modeled it as a very bad stack machine, but since all of the expressions only had two
terms/factors, I just allocated a 2-item array in-place.

The second was the little shortcut I found for part 2. Essentially, part 2 is just part 1, but
bigger. You simulate for 10,000 rounds instead of 20, and remove the 1/3 damping parameter. The
numbers get real big real fast. The first time I ran it it panicked.

The first thing that came to mind was just using a `BigInt` style interface. Rust doesn't have that
natively like (modern) JavaScript does. There are crates that do it, but I really want to write all
the code for this year myself, which means I'd have to implement it myself. That sounded like no
fun, and I was tempted to just not do part 2.

Then I realized: the actual worry values for the items aren't used. They're only there to keep
track of which monkey to throw the item to. Modular arithmetic to the rescue! The only property of
the number we need to maintain is the divisibility by a modulus. That property is preserved if you
modulo the number, as long as the new modulus is also divisible by the old modulus. All I had to do
was multiply the moduli of all the monkeys together to get a big modulus that will preserve
divisibility for all the monkeys' individual moduli. By this point, I was so tired of the challenge
that I just hacked it in manually rather than keeping track and setting the "big modulus" on all
the monkeys.

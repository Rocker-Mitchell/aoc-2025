Notes and takeaways from doing AoC with the codebase prepared for Rust:

Structuring a Cargo workspace was likely overkill, but did usefully separate
concerns of CLI, framework, and solutions.

- Didn't need separation of dependencies that much, as I'm solo managing the
  uses of dependencies.
- Linting configuration was across files, cascading by tree module structure;
  that's reasonable to do in one crate.
- Conceptually I wanted the framework and solutions like their own libraries,
  but they probably could have been two modules exported in library.
  - But if I get a crazy idea to share the framework as a published package...

The architected framework code had both appealing and uncomfortable takeaways.

- Breaking down the anatomy of solutions to multiple traits felt great, like a
  solution was a composition of features rather than one thing to inherit.
- Parsing returning a specific type of error was limiting. I thought it'd be a
  few error enum types to add as needed, but it got to a point that errors
  specific to solutions were so unique that it was easier and sensible to use
  panic messages instead of adding to the parse error enum.
  - For another time, I'd have a return of `Result<T, Box<dyn Error>>`.
    Implementing a solution should return any error it wants. The library can
    provide some commonly useful errors like empty input, invalid line, et
    cetera, but solutions can implement and return their own errors.
  - Maybe part methods could have returned errors as well, if more desirable
    than converting standard library errors to panics.
- I think having traits for parts implement how to run their specific feature &
  a `run()` method for a full runner wasn't appropriate, especially with `run()`
  mentally overlapping with the `RunnableSolution` trait having that method as
  its core. For another time, I'd want the runner logic for the traits &
  implementations to run full solutions handled as generic functions outside the
  traits; the functions can specify expected trait bounds, and the
  `impl_runnable_solution!` macro can call the appropriate function. If a
  solution wanted to change how it runs, it could've implemented
  `RunnableSolution` directly instead of leaning on the macro.
  - The traits could probably use less supertrait patterns, if the generic
    functions can specify the collection of traits it will look to use.
- Having parsing traits require a type for parsed input &mdash; and passing a
  ref to that type for parts &mdash; was okay, but I wonder if not having the
  parse type public could be convenient. Kinda rubs me that any custom struct I
  formatted parsed data into always had to be made public.
  - If parsing was implemented as constructing an instance of the solution
    struct (returning `Self`), then the parsed data would be private properties.
    Having a tuple parse result would adapt to multiple properties instead.
    Trait methods for parts would expect `&self` so the immutability of the
    parsed data could be enforced for safe data sharing between parts.
    - Maybe it would just be some trait for constructing an instance from input.
      Solutions internally decide what properties to construct, either raw input
      or processed input. Part traits could simplify and be used in either parse
      style. But then how's the output handler properly invoked? And what's the
      appropriate way to handle raw input: copy, track ref with lifetime,
      something else?
- Having the solution runner use a struct implementing an output handler trait
  so messages print as the solution runs was great! Very flexible for a CLI to
  format messages as desired.
- Got an idea later about a separation of concern for parsing and parts.
  `ParsedPart1` involved specifying a type for parsed input so the part's
  function could expect it as an argument; nothing restricts that type
  from being a string of the raw input. So, instead of two traits for a part
  where the difference is a parse function being available (`ParsedPart1` and
  `Part1`), there can be a trait for parsing input into usable data and a part
  trait adds a type definition to be expected as the part function's argument.
  - The parsing trait would likely do a constructor pattern, take in a string
    ref to return an instance of self.
  - If no supertraits are leaned on, then the struct implementing the parse
    trait can be separate from the part traits. One trait doing
    everything would still work, but separating cognitive load of a struct
    representing data vs implementation would be useful.
    - Runner generic functions would need to outline multiple generic types
      for the separate traits, as they couldn't/shouldn't require one type to
      implement all traits.
  - Some types for parsed data were just standard library collections, like
    vectors. If the parse trait must return an instance of self, a technique
    could develop of tuple structs that wrap the collection, like
    `struct DayNData(Vec<u16>)`.
  - If the parts don't require parsing and want a string ref passed, runner
    generic functions can be defined that pass the string ref instead of
    parsing first.
  - Trait bounds would be significantly leaned on in generic functions. With a
    trait `Part` that defines `type Input` and `fn part(input: &Self::Input)`:
    `<P: Part<Input = I>, I: Parse>` requires the part expects the parsed input
    type, `<P: Part<Input = str>>` requires string input so a raw input string
    can be passed.
    - That sparks a possible idea that a single trait can be done to represent
      a part to solve. But getting this to allow one struct to implement twice
      for two parts would be done with a generic trait, and some marker type
      to differentiate the parts implemented. If there's a `Solve<P>` trait and
      empty structs `Part1` and `Part2`, then implement blocks can target
      `Solve<Part1>` and `Solve<Part2>`.
      - Notably found if the trait implements `fn solve(&self, input: &str)`,
        the struct can be constructed then methods invoked with
        `Solve::<Part1>::solve(&solver, input)`.
- There's an older package maybe worth looking into:
  [advent_of_code_traits](https://crates.io/crates/advent_of_code_traits)

Notes on implementing solutions as AoC started:

- There were significant unknowns of if there are packages appropriate for
  solutions. I had serviceable predictions for using nalgebra for cartesian
  points and grids, but incorrectly predicted any solution requiring regular
  expressions, and didn't predict needing a package to allow ordering floats.
  I've not been exposed to CS-grade problems to solve in Rust before, so
  devolved to scrambling for anything to just get something working within the
  time I set aside for the day, without any experience per package or knowing
  if the standard library would be enough (or the problem like with ordering
  floats). There's no good solution to this besides actually getting experience.
- Type aliases were handy. Especially if I got an integer overflow bug, then I
  only change the alias and all uses update to the larger integer type.
- Panics during parsing or parts were welcome, but was most useful to mark code
  that is expected to be internally valid so any error means the code's
  assumptions were wrong. A good case was checking for overflows doing math on
  integers, as an overflow usually meant a larger bit representation was needed.
- It's fantastic that it's easy to write unit tests per solution, and that I can
  check correct behavior with the example inputs from the site. Last year, my
  pattern was to have test text files to run against, but with unit tests I
  didn't need custom text files to pass and could do continuous integration if
  I refactored working solution parts.

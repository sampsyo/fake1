Traditional Make pattern matching looks like this. You can only refer to one field. So one (somewhat funky) way to write a rule for compiling LaTeX documents to PDFs is:

    %.pdf: %.tex
        pdflatex $*

In Fake, pattern variables have names so you can refer to different parts of the pattern unambiguously. You refer to pattern variables like `{this}`. Conveniently, the first Make-style `%` placeholder is named `%`, so the legacy `$*` is syntactic sugar for the Fake-style `{%}`:

    %.pdf: %.tex
        pdflatex {%}

You can also make the pattern more explicit in Fake by giving it an English name:

    {name}.pdf: {name}.tex
        pdflatex {name}

That example demonstrates that the `{curly}` syntax represents *quoting* in Fake. You can write Fake expressions inside the braces. You can think of this similar to `#{e}` interpolation in Ruby strings if that's comfortable.

You can also skip this convenient string-interpolation syntax and write Fake expressions directly. For familiarity, these interpolated chunks are syntactic sugar Fake expressions that use `interp` to interpolate the quoted parts of the string. Targets and prerequisites desugar to `File` expressions, meaning that they refer to an existing file on the filesystem, and recipe lines desugar to `Shell` expressions, indicating `sh` code to execute.

You can write the desugared versions of these three expressions using parentheses, which indicate a raw Fake expression rather than a string:

    ( File (interp "{name}.pdf") ): ( File (interp "{name}.tex") )
        ( Shell (interp "pdflatex {name}") )

The `interp` above is a macro (TODO: function?) that in turn expands to concatenation. (TODO: Is this defined in a library or is it magical? Do we really need general macros? And how about `Str`: that's magical, right?)

    ( File (Str name ".pdf") ): ( File (Str name ".tex") )
        ( Shell (Str "pdflatex " name) )

These elaborated expressions show the essence of values in Fake: they are S-expressions. The first position in a value expression is a constructor, which is always written Capitalized. Lower-case first positions are function calls. Fake provides pattern-matching for these trees along the lines of ML or any other functional language worth its salt.

To see the similarity with an ordinary functional language, we can desugar this Make rule syntax further to a function definition. In general, a Make rule like this:

    target: dependency
        recipe

is sugar for the Fake function declaration:

    let make target = Plan (make dependency) recipe

This declaration says, *To make `target`, first recursively try to make `dependency` and get its plan. If that succeeds, return a new plan that glues the result together with `recipe`.* This easily generalizes to multiple dependencies and multiple recipe steps.

The user interacts with this Fake program by calling `make something`, which returns a full plan tree for making `something`. The Fake program then executes all the steps in the complete plan.

To make this useful, we need pattern matching to work a bit differently than it does in most functional languages: it needs *backtracking*. In ordinary languages, a call `f x` can fail if the argument `x` matches none of the declarations for the function `f`---and a failure like this halts the program. In Fake, a match failure *propagates* to the calling function, causing its rule to fail. The backtracking pattern matcher can then try the next rule for the calling function.

TODO: There needs to be some manner of fallback rule for `make (File s)`. And some universal semantics for `File` (or in general?) to implement newness detection...

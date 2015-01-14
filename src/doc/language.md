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

The `interp` above is a macro (TODO: function?) that in turn expands to concatenation. (TODO: Is this defined in a library or is it magical? Do we really need general macros? And how about `Cat`: that's magical, right?)

    ( File (Cat name ".pdf") ): ( File (Cat name ".tex") )
        ( Shell (Cat "pdflatex " name) )

The essence of Fake's pattern matching is clearer in this desugared form. Fake expressions are just S-expressions. Pattern matching works by trying to align these trees.

Pattern matching works much like Make but for trees instead of strings:

1. Take as input a *goal expression*. For example, a goal expression `( File "paper.pdf" )` indicates that we want to build the file `paper.pdf`---our goal is for that file to exist.
2. Look through the list of rules in the cookbook and try to match (one of) each rule's *taget expression* with the goal.
3. For the matching rule's *prerequisite expressions*, recursively treat each as a goal and find a matching rule for each. This builds up a *dependency tree*. (TODO: What about cycles?)
4. Now that we have a tree, execute each recipe from the bottom up. (TODO: How to express timestamp-based re-making?)

Maybe one further desugaring step can get rid of the standard Make target-prerequisites-recipe syntax?

    func pat -> res

    func target -> Recipe( prerequisite recipe )

    rule File (Cat name ".pdf") ->
        Rule( ( File ( Cat name ".tex" ) )
              ( Shell ( Cat "pdflatex " name ) ) )

Or maybe it's:

    make target -> ( make prerequisite ) ++ recipe

    make File (Cat name ".pdf") ->
        ( make ( File ( Cat name ".tex" ) ) )
        ++ [ Shell ( Cat "pdflatex " name ) ]

Where there's some manner of backtracking search. If any of the `rule` calls fail, that bubbles up and causes the parent match to fail. The pattern matcher then has to find another rule for the outer match.

TODO: How to represent multiple prereqs and multiple recipes?

So `Big` is a constructor and `little` is a pattern-matched function call? Perhaps the algorithm goes like this:

    actions = []
    def algorithm(target):
        for prerequisite, recipe in make(target):
            if algorithm(prerequisite):
                actions += recipe
            else:
                return False  # Could not satisfy prerequisite.
        return True  # Satisfied everything.

    make(target) -> make(target, [])
    make(target, actions) ->
        match rule(target) with
            Rule(prerequisite, recipe) -> 
            | nomatch -> xxx
    make(null, actions) -> actions

TODO: Feels like we need to re-implement pattern matching to deal with the prereqs side of the equation. Or, rather, the above attempt to use the pattern-match needs to get all the matches in order and discard them, which should be pattern-matching's job.

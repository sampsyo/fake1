## Syntax

Expressions `e` have primitive literals (symbols and strings), tuple construction, and match invocations written `$e`:

    e ::= symbol
        | "string"
        | (e*)
        | $e

I'm using an asterisk as a Kleene star.

A program is a statement sequence. Statements add bindings to the language:

    s ::= symbol <- e
        | p -> e
        | s ; s

The two types of bindings are eager (the `<-` form, where the pattern must be a symbol---mnemonic "gets") and lambda-style (the `->` form, where the pattern `p` is general---mnemonic "function").

Patterns follow expressions closely but do not have invocations. Instead, they add binding names, written like `:this`:

    p ::= symbol
        | "string"
        | (p*)
        | :symbol

Unsurprisingly, run-time values are just tuple-trees:

    v ::= symbol
        | "string"
        | (v*)


## Examples

Just an expression, using a built-in binding for `print` that makes it look like a function:

    _ <- $(print "Hello, world!")

The same, but via a user-defined "function":

    (greet :s) -> $(print $(cat "Hello, " $s "!"));
    _ <- $(greet "world")

That tiny example reveals three unfortunate aspects of the syntax:

* The semicolon, which should really be whitespace.
* The extra parentheses on the LHS of the binding. A tuple should probably be implied, and a single-element tuple should be equivalent to an atom.
* Expressions evaluated only for their side effects still need to bind to some name. We could could consider allowing bare expressions.

Maybe these can be hidden with sugar.

One more step shows how bindings work like variables:

    who <- "world";
    _ <- $(greet $who)

The left-arrow binding isn't really material this time, but it decides when the expression is evaluated. Since the language is dynamically scoped, "when" matters.

Here's a simple Make rule in Fake:

    (make (file (str :name ".pdf"))) ->
        (seq
            (par
                $(make (file $(cat $name ".tex")))
            )
            (shell $(cat "pdflatex " $name))
        )

The construction `(shell ...)` demonstrates a peculiarity that gives us delayed execution for free. Since there is no `$`, we do not "execute" the `shell` "function" when matching this rule. (Scare quote overload intended.) Instead, we wrap it in a `plan` tuple and return it. Later, we can execute it using `$`.

Here's a simpler example of delayed execution. First, this program just emits the tree `(print "Hello!")`:

    greeting -> (print "Hello!");
    _ <- $greeting

This program, on the other hand, actually prints "Hello!":

    greeting -> (print "Hello!");
    _ <- $$greeting

Fake is beginning to look like Lisp but with pattern-matching instead of functions.

How do you write the Make algorithm now? Are there special varargs rules for `seq` and `par`?

    (seq :a :b :c ...) -> (seq $a $b $c ...)
    (par :a :b :c ...) -> (par $a $b $c ...)  // but in parallel
    _ <- $$(make (file "paper.pdf"))

TODO: This lets us emit and execute plan *trees*, but a real Make requires *dags*, so that one product can be used as a prerequisite in multiple places. It would also be nice to support one action producing two files.

# mdanchored

mdanchored is a simple Markdown formatting helper. It moves markdown link
references to the end of their current "section". A section ends at:

1. A heading.
1. The end of the document.
1. A line containing `<!--more-->` (this is a [Hugo-ism])

Input is passed via `stdin` and reformatted output is sent to `stdout`.
Optionally, passing the `--deno` flag will further format the document with
`deno fmt`, which is a fast formatter that works in a way I like.

This is purely a personal tool to help me write markdown and have it formatted
the way that I like it. I don't expect it to be that useful for anyone else. It
also allowed me to write a first Rust application 🎉

[Hugo-ism]: https://gohugo.io/content-management/summaries/#manual-summary-splitting

## Example

Taking this example, with `[link]` between two paragraphs:

```
My paragraph is very
long.

[link]: http://foo.com

And another paragraph, and it
feels so wrong that there's a
link in the middle.

## A heading

I love hanging out below this
heading.
```

We would end up with `[link]` instead above the heading:

```
My paragraph is very
long.

And another paragraph, and it
feels so wrong that there's a
link in the middle.

[link]: http://foo.com

## A heading

I love hanging out below this
heading.
```

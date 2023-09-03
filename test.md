---
title: Journal 2023-08-29
date: 2023-08-29
draft: false
tags:
  - journal
---

I thought I'd take a stab at quick-fire journal entries.

First, we return to Helix and take a look at Kitty.

Then, a note about ClickHouse. Getting hands on with ClickHouse has helped me
understand in practice what I previously only understood in theory
(column-orientated datastores). In doing so, it's expanded my horizons of what's
possible.

<!--more-->

### Helix

I [wrote about Helix][hx] back in June. While I liked it, I wondered whether I'd
continue to use it. It turns out that I have done, for both work and for home.
I've used Helix to write non-trivial amounts of Go, and all the posts on this
site since June.

I still find the Kakone-like select-and-act paradigm a nice one, and I am
finally starting to internalise some of the main actions.

One problem that I had was spell-checking in Markdown. I've solved this by using
the [LanguageTool][lt] language server, [ltex-ls][ltexls]. I used [this post] to
integrate LanguageTool with Helix. It's somewhat heavyweight, and I'd love
[this PR][spellpr] to land, or something like it.

I still miss sentence-wise text motions.


[lt]: https://github.com/languagetool-org/languagetool
[ltexls]: https://github.com/valentjn/ltex-ls
[hx]: https://dx13.co.uk/articles/2023/06/15/helix-editor/
[spellpr]: https://github.com/helix-editor/helix/pull/6343
[this post]: https://blog.getreu.net/20220828-tp-note-new8/

### Kitty

When migrating back to terminal editors, I realised I needed a better terminal
emulator than the macOS terminal. This is because macOS terminal will display
very few colours, meaning that nearly every editor colour theme looks like ass.

First I tried [iTerm 2]. It seemed like the most used, but I found it quite slow
and, for my tastes, ugly. Next I tried [Alacritty], which seemed fast
and good-looking, but I found it difficult to configure. Finally, I found
[Kitty].

I like Kitty. It's as fast as Alacritty (although it's a subject much debated by
the communities). Importantly, I found configuring it to be relatively easy, and
the documentation to be very good. It's easy to theme the shell too. Overall,
recommended.

Helix doesn't have an in-built terminal emulator, so I've learned Kitty's
hotkeys for creating new shell windows within each tab. Kitty has a bunch of
power in the form of pre-defined [layouts]. These define how Kitty automatically
arranges your windows/panes as you open new ones. I've arranged things so Helix
takes up the top 75% of the Kitty tab, while new terminals squeeze their way
horizontally along the rest of the space. Kitty calls this the "Fat Layout". It
works well for the "main app and supporting terminals" use-case that I find
myself in a lot.

![](kitty.png)


[iTerm 2]: https://iterm2.com/
[Alacritty]: https://alacritty.org/
[Kitty]: https://sw.kovidgoyal.net/kitty/
[layouts]: https://sw.kovidgoyal.net/kitty/layouts/

### ClickHouse

I've been using [ClickHouse] in a proof-of-concept. Using column-orientated
databases is new to me, and the speed, at least on my MacBook Pro M1, feels
pretty breathtaking. ClickHouse will scan down an integer column to find latency
percentiles at well over 100 million rows a second. Clearly putting the column
data rather than row data next to each other on disk pays dividends for the
right use-cases. Who'd have thought? 🙃

I'm not sure whether I will end up using the database, but trying ClickHouse has
opened up new vistas in what's possible with online analytical processing for
me. That's a good thing to find out, as it expands my understanding of what's
possible to build with (relative) ease. A tool for the belt.

I found these two articles from CloudFlare particularly useful in understanding
the raw abilities of ClickHouse before deciding to proof-of-concept with it:

- [How Cloudflare analyzes 1M DNS queries per second][ch1]
- [Log analytics using ClickHouse][ch2]

That's it for now!


[ClickHouse]: https://clickhouse.com/
[ch1]: https://blog.cloudflare.com/how-cloudflare-analyzes-1m-dns-queries-per-second/
[ch2]: https://blog.cloudflare.com/log-analytics-using-clickhouse/

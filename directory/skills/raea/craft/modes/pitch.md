# Mode: pitch

Ads, promos, launch spots, app store previews: pieces whose job is that someone stops, believes one thing, and does
something.

## It works when

A viewer who had no plan to watch stops, believes the brief's `believe:` line, and knows the one thing to do next.

## What to make

- **The hook in two seconds.** The first line, said and shown, lands the reason to keep watching. A logo, a greeting
  or a slow push on nothing is not a hook.
- **One belief.** The whole piece argues the brief's `believe:` line. A second idea halves the first.
- **One proof only this product has.** Show the product doing the thing, at the moment the voice claims it: the real
  screen, the real result, the real person. Telling without showing doesn't convince anyone.
- **One action at the end.** The ask is on screen long enough to act on, in its working form (the name, the link,
  the handle), inside the safe areas.
- **Specific, not generic.** Cover the logo and ask whether another product could run this unchanged. If it could,
  the angle isn't specific enough yet.
- **Plan it.** Hook · problem · proof · ask as a storyboard, approved before anything renders. Chain its cuts on the
  thirds so the proof lands where the problem was (`raea guide match-cuts`).

## A client's brief is a contract

When a client hands you a brief (a PDF, a doc, an email of rules), make it rules before you plan a single cut, or it
is only context, and context gets lost. Run `pdftotext -layout BRIEF.pdf media/brief.txt`, then write
`make: cut({ cut: {...} })` on brief.ts from it:

- `count`, `seconds`, `formats` and every cut (id, angle, solo or mashup, hook, captions or a text hook);
- `say`: the phrases the brief asks every cut to say or show (`every: false` when one cut is enough);
- `never`: the claims, words and names it bans;
- `cta`: the exact line every cut ends on;
- `open`: each [TO CONFIRM], gap or contradiction in it, as a question for the review.

Every rule carries `from`, the brief's own words for it, copied exactly. `raea make status` finds each one in
media/brief.txt, so a rule can't be made up and a line can't be paraphrased away. Then it holds every file at
out/cuts/ID_WxH.mp4: its shape and length; its words (FILE.words.json, newer than the file: raea cloud writes it
after every render) for each `say`, `never` and the CTA; and its frames check (FILE.frames-check.json, newer than
the file: raea cloud's measured faces and text sizes; `frames: false` where there is none). The job isn't done, and
the Stop hook won't let you end, until every gate is clear and every `open` item has the person's answer, written in
with who gave it.

## The decisions

Each one has a default that comes easily. Question it before you take it.

- **The belief.** Write the `believe:` line the way a viewer would repeat it to a friend. The easy default is a
  feature ("it has AI editing"). A belief is what changes for the viewer ("my edit ships the same day").
- **What opens.** The easy default is problem first, then proof. When the proof is striking enough to stop someone
  scrolling, open on the proof, and let the problem be the reason it matters.
- **What carries it with the sound off.** Feeds play muted. The picture and the burned-in words make the argument,
  and the voice adds to it. A piece that only the voice explains fails the mute test.
- **The length.** Use the delivery's length. Each cutdown is its own edit with its own hook (`motion/delivery`). The
  easy default is the long cut stopped early.
- **The ask.** Use the action the brief names: install, try, book. The easy default, "learn more", only makes sense
  when the brief has no real action.

## The rut

The template ad: white captions with one yellow word, a zoom on every sentence, a whoosh on every entrance. A big
number with nothing behind it. Glass cards floating over footage.

## How it fails

| What you see | Why | What to change |
|---|---|---|
| people leave in the first seconds | the opening is a logo, a slow push or a setup line | open on the proof or the sharpest line |
| viewers remember the ad but not the product | the proof is generic: stock footage, a mood, a claim with nothing shown | show this product doing its thing, on screen, as the voice says it |
| it reads as a template ad | a zoom on every sentence and a whoosh on every entrance | keep motion and sound for the two or three moments that carry the argument (`quieter`) |
| the ask goes by unread | the end card is up for less than two seconds, or sits under a platform's buttons | hold it long enough to act on, inside the safe zones |
| a claim nobody can stand behind | a number, quote or result that isn't in the brief | remove it, or ask the person for the real one |

## Check

- Mute it: does the picture alone make the argument?
- Watch only the first two seconds: would you stop?
- Every claim is one the brief or the person gave you.

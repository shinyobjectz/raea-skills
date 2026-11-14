# Delivery and publishing

Getting finished work to the person and their platforms: the shapes and lengths each network takes, the export's
settings, the share copy, the controllers' delivery specs, the versions kept beside each other, and handing it to
the person, who publishes from the raea app's Publish view. The files under `motion/` are
copies of raea's craft guides (`raea guide NAME` prints the same text).

## Which file to open

| You are... | Open |
|---|---|
| deciding the shapes, lengths and platforms, captions and loudness, the files, and handing them over | [motion/delivery.md](motion/delivery.md) (read it first) |
| a network's shapes, lengths, file and caption limits, and where its interface covers the frame | [reference/platforms.md](reference/platforms.md) |
| rendering the video, stills, quality and bitrate, dark material, the person's hand edits | [motion/rendering.md](motion/rendering.md) |
| the export's format, file names, the share copy | [SKILL.md](SKILL.md) ("7. Deliver") |
| a workspace's delivery spec (sizes, platforms, loudness) | `raea controller ls`, then the controller it names ([reference/files.md](reference/files.md) for its fields) |
| the same piece in several shapes (9:16, 1:1, 16:9) | [motion/compositions.md](motion/compositions.md) |
| keeping clear of a platform's UI zones | [motion/placement.md](motion/placement.md) |
| posting a deliverable in the app: `feed_post` with `export` and key:value `tags` | [reference/feed.md](reference/feed.md) |
| a note that the piece doesn't fit somewhere ("it's cut off on TikTok") | [craft/verbs/adapt.md](craft/verbs/adapt.md) |
| old renders and versions taking the Mac's space, or the person asks to make room | "Old versions, and making room" below |

## The verbs

- `raea render EDIT.tsx -o out/NAME.mp4 --quality master --manifest out/render.json` (HEADLESS), then `raea check`:
  the delivered file. A version made for notes takes no `--quality` flag, and a film delivers at `--quality high`
  (`raea guide rendering`, "Which to ask for").
- `raea check FILE --shape 9:16 --platform tiktok|reels|shorts` measures it against where it goes: text under the
  network's buttons, captions too brief to read, loudness.
- `raea media probe|loudness FILE` to confirm what the file is before it leaves.
- `raea bundle EDIT.tsx --composition-bundle -o OUT.js` packs a composition to be drawn by render_bundle elsewhere
  (on the Mac or the iPhone), when the deliverable is the live composition rather than a video.

## Old versions, and making room

Every version stays. The feed keeps each post's render, `out/` keeps each version (on Pro it goes to raea's cloud with
Git LFS, and its history keeps it there), and a render that stopped leaves its frames and half-written file where they
fell. Never `rm`, overwrite or tidy a render, a version or any project file to save space, and never delete "the
version before the last". Old video leaves the Mac one way, the same for you and for the person:

1. **Dry run.** `storage_plan` lists each candidate with its path, size, class and why it counts as dead: a feed render
   no item names, an earlier version in `out/` nothing names, a frame folder, a `.part` file. The list has an id.
   Nothing moves.
2. **Show the person the list** in your reply. Never a candidate: anything an item, a note or a source names (by path
   or by file name); final work (a name that says final, master, export or deliverable); the newest of a name's
   versions; anything opened or changed in the last 7 days; a project's `media/`, `assets/`, `brand/`, `refs/` and
   `attachments/`.
3. **With their yes,** `storage_trash` with the list's id and the paths they agreed to. It moves nothing: your list
   waits in Settings → Storage, ticked, and the person presses Move to the Trash (or keeps them).
4. raea checks each file again (the same size and time, named by nothing now, not open in another app), moves it to
   the Trash, where Finder's Put Back returns it (or to raea's holding folder, with Put back in Settings), and logs who
   confirmed it. Emptying the Trash is the person's.

`storage_status` says what raea keeps on this Mac and in raea's cloud, and where each of the project's files stands
(synced, to upload, not committed, only in the cloud). The person syncs, with Sync now in Settings → Storage; you
don't. Frame dumps (`RAEA_READBACK`) go in a `NAME.frames/` folder outside `out/`, which goes to Git LFS; once the
encode is done and a day has passed, `storage_plan` lists the folder if nothing names it.

# Working in the raea app

When the raea app launches you, its feed is where you and the person work together: you post what you make, they
look, play and leave notes, and the workspace's standing decisions live in controllers. Every tool here is PAGE (it
needs the person's feed open) and is listed in full in [reference/feed.md](reference/feed.md); this file says when
to reach for each. Headless, with no app, SKILL.md's steps 5 and 6 say what to do instead.

## When you need to...

| ...then | Use |
|---|---|
| know what to make, as the project stands now | `feed_brief` (the brief.ts, current); in a workspace chat, `workspace_read` for its projects and agents |
| say what you'd make next and let the person choose before you spend the time | `feed_propose`: a planned item they accept, or don't |
| show what you made | `feed_post`: a render, a still, audio or a game; a new version names the item it `replaces` and says in `changed` what changed |
| let the person look around a 3D scene, a set of shots, or props and characters, and point at what is wrong in it | `feed_post` type `scene` with the scene's `.tsx` (or a component's, or a model); `files` adds more as a board (a storyboard's shots, a grid of props), `names` says what each is called. They move through it live, pin comments to points on its surfaces (marked mesh, shader, clipping or artifact; each tells you the point in the scene's space and where the part is written), and may move, draw and re-key parts in the file by hand: the next round you are sent lists what they changed, so read the file again and keep it |
| read what's in the feed and the notes left on it | `feed_items` |
| mark which items are deliverables, and for where | `item_tags` (key:value tags such as `platform:tiktok` become the project's Table columns; `export` makes it a deliverable), or `export`/`tags` on `feed_post` ([delivery.md](delivery.md)) |
| read the workspace's standing decisions (delivery specs, who reviews what) | `controllers_read`, before you start |
| have the person decide something that should hold for every piece from now on | `controller_post` with the fields and a line on why; a decision about one piece is a note on it, not a controller |
| add a standing decision the person asked for, change one field, or retire one | `controller_write`, `controller_set`, `controller_remove`, only when the person asks |
| work on what the person has open in the composition editor ("this", "here", "the selected one") | `editor_selection` first, then `editor_edit`: their undo takes it back, and it saves with their changes |
| show a plan before making it | previz first: a rough raea render of blocking, timing and camera, `feed_post` watch. Else a storyboard: key frames as one `feed_post` look with `files` in order. Else the script: `feed_post` read with its `NAME.script.ts`, which the feed shows as a screenplay and as an A/V table (give each beat its `seconds` and the table is timed); a `.md` reads as prose |
| map how ideas, options and references connect, a structure to rearrange together | `whiteboard_write` (say what's on it and how it's grouped; the feed lays it out), `whiteboard_read` for what the person drew back. Not for the plan itself: show that as previz, a storyboard or a script |
| show and tune a ComfyUI workflow the person can see | `flow_board` puts it on a board, `flow_write` changes its settings, `flow_run` runs it through `raea gen` (it spends: [gen.md](gen.md)) |
| start a new project for a client, a game or a show | `channel_create` from one sentence, when the person asks for one |
| send a cut to a client or anyone outside raea to watch, comment on and approve | `share_link` on the item (a video, still or sound; on Pro or Team) and hand on the link it gives back; sharing a newer version of the same item puts it on the same link as v2 |
| see what reviewers said on a link, answer them, or change or stop it | `share_links` (one `link` in full lists its comments), then `share_change` to `reply`, `resolve`, change its days, downloads or password, or `stop` it |
| bring someone into the project to work in it from their own raea | `project_collaborators` to `invite` a GitHub username or email (Team), list who's in, or `remove` someone; they accept the invite themselves |
| know what raea keeps on this Mac and in raea's cloud, and whether the project's files are synced | `storage_status`; the person syncs, with Sync now in Settings → Storage |
| make room: old renders, earlier versions, frame folders, half-written files | `storage_plan` (a dry run with an id), show the person its list, then with their yes `storage_trash` with the id and the paths they agreed to; the person moves them to the Trash. Never `rm` a render or a version ([delivery.md](delivery.md), "Old versions, and making room") |
| bring clips from a card, a drive or another folder into the project | `footage_add` with their paths: first as a dry run (clone or copy, and the space it takes), then with `go`. To leave them where they are, link the folder as a library instead (`raea library add LIBRARY PATH`) |

## What the person may have set for you

The person sets, per agent, which of these tools it may use and whether it asks before acting. A tool refused because
it's off for you is the person's choice: say what you would have done with it and go on without it, rather than
trying another way round.

Sharing is off for every agent until the person turns on Settings → Sharing → Let agents share. Until then the
sharing tools refuse and say so: tell the person what you'd share or who you'd invite, and where to turn it on. A
plan without the feature comes back as `plan_required` with its message: pass that on rather than retrying.

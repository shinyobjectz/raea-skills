# Planning and tasks: the project's plan.org

Every raea project keeps its work in one file, `plan.org` at the top of its folder. It is plain Org mode text (Emacs's
outline format), and it is the to-do list the person sees in the raea app. You keep it: plan in it before work of
more than one step, mark each task as you start it, close it with what you made, and add what you find left to do.
Do this whether or not the person ever opens it; a plan nobody maintains is a blank list, and a blank list tells them
nothing about where their project stands.

Not the same file as a solid's `plan.ts` (the build plan a procedural object is checked against): `plan.org` is the
project's tasks.

## Where it is, and how you reach it

- In the raea app (it launched you): the plan tools, `plan_read`, `plan_add`, `plan_update` and `plan_close`
  ([reference/feed.md](reference/feed.md)). Each turn's prompt also carries `raea://plan/<project>`: the open tasks by
  id, and which task the person's message is. Use the tools rather than writing the file yourself, so the person's
  list changes at once.
- Anywhere else (a terminal in the project's folder, a headless run): open `plan.org` and edit it as text. Keep every
  `:ID:` as it is, and give a new task one (`t-` and eight lowercase letters and digits; the app gives one to a task
  that has none the next time an agent reads the plan). The app reads the file again while the person's to-do list is
  open and before each of its agents' turns, so they see your changes. With no `plan.org` yet, start one with the head
  below.
- Find the folder with `raea project --json` (SKILL.md section 0). The plan sits beside `brief.ts` or `brief.md`.

## What a plan looks like

```org
#+TITLE: Stay
#+TODO: TODO MAKING REVIEW | DONE DROPPED

* Look
** MAKING [#A] Light the oil door [1/3]                                    :look:
DEADLINE: <2026-11-20 Fri>
:PROPERTIES:
:ID:       t-k3x9q2ab
:AGENT:    a-light
:FROM:     m-mwgv7a
:END:
- [X] warm key from the lamp
- [ ] cool fill from the window
- [ ] rim on the handle
** DONE Grade the hall
CLOSED: [2026-11-14 Sat 16:40]
:PROPERTIES:
:ID:       t-k3x8m1zz
:ITEM:     i-hall-2
:END:
Graded warm; the window stays cold. [[item:i-hall-2]]
* Sound
** TODO Score the door opening
:PROPERTIES:
:ID:       t-k3xa0c4d
:END:
```

- **Headings without a keyword are stages** (`* Look`, `* Sound`): they group the tasks under them. A task is a
  heading with a keyword.
- **Keywords.** `TODO` not started, `MAKING` under way, `REVIEW` waiting on the person, `DONE` finished, `DROPPED` no
  longer wanted. The `#+TODO:` line spells them out for Emacs. A plan may add its own sequence on another `#+TODO:`
  line; raea reads it.
- **Priority** `[#A]`, `[#B]` or `[#C]`, after the keyword. Use A for what blocks everything else, sparingly.
- **Tags** after the words, between colons: `:look:sound:`. Letters, digits and `_`: write a dash as `_`
  (`act_one`). A task inherits its stage's tags. The feed's views and an item's tags read them.
- **Dates** on one line right after the heading, the planning line: `DEADLINE: <2026-11-20 Fri>` when it's due,
  `SCHEDULED:` when to start it, and `CLOSED: [...]`, stamped when a task closes (in the app the tools stamp it). All
  three share that one line; Org reads no second planning line, and a drawer under one is lost to it.
- **Properties**, in the drawer: `:ID:` (every task has one, and it never changes), `:ITEM:` the feed item it stands
  on or made, `:AGENT:` who has it by agent id (an `@name` while no agent has that name), `:FROM:` what asked for it
  (a message id, or `ITEM#COMMENT` for a comment). Anything else you add is kept.
- **Links** by id, never by title: `[[item:i-hall-2]]` for a feed item, `[[id:t-k3x9q2ab]]` for another task.

## When to plan

- **Before work of more than one step**: write its steps as tasks under a stage, each one small enough to finish in
  a turn or two. One-step asks need no plan of their own, but the message is already a task (the composer writes it,
  `:FROM:` the message), so mark and close that one.
- **At the start of a project**: its stages (Look, Script, Make, Sound, Deliver, or what fits), with the first few
  tasks under each. A long plan nobody will reach is noise: plan the next stretch in detail and the rest as stages.
- **When you find more to do** (a fix a critique asks for, a shot you need, a question for the person): add it where
  it belongs, instead of keeping it in your head or in a chat message.

## As you work

1. **Start**: read the plan (`plan_read`, or the turn's plan block). Find the task you're on, or add it.
2. **Mark it `MAKING`** as you begin (`plan_update` with `state: "making"`). Tick its checkboxes as each step is done
   (`check: N`, 0 first).
3. **Waiting on the person** (a pick, an approval, a note): mark it `REVIEW`, and put the item they're to look at on it
   (`item`).
4. **Close it** as you finish, in the same turn: `plan_close` with the item you posted (feed_post's answer) and one
   line on what was done. That stamps `CLOSED` and links the work. Outside the app: change the keyword to `DONE`, put
   `CLOSED: [YYYY-MM-DD Day HH:MM]` at the start of the planning line (make one right under the heading when it has
   no DEADLINE or SCHEDULED), and put the item's id in `:ITEM:`.
5. **No longer wanted**: `plan_close` with `state: "dropped"` and why. Remove a task only when it was a mistake; a
   dropped task keeps the record of what was decided.
6. **Before you end the turn**: no task of yours is left `MAKING` that isn't still under way.

A task linked to a feed item stands where the item does: when the person approves it or a test passes, the list shows
it done. You still close it in the plan, so the file says so too.

The calls, as the app's tools take them (every task by its id, from `plan_read` or the turn's plan block):

```text
plan_read   {}                                    or {"full": true} for the whole file
plan_add    {"tasks": [{"title": "Light the oil door", "under": "Look", "checks": ["warm key", "cool fill"]},
                       {"title": "Rim on the handle", "under": "t-k3x9q2ab"}]}
plan_update {"changes": [{"id": "t-k3x9q2ab", "state": "making"}, {"id": "t-k3x9q2ab", "check": 0}]}
plan_update {"changes": [{"id": "t-k3x9q2ab", "state": "review", "item": "i-door-2"}]}
plan_close  {"id": "t-k3x9q2ab", "item": "i-door-3", "note": "Warm key at 3200K, cool fill from the window."}
plan_close  {"id": "t-k3xa0c4d", "state": "dropped", "note": "The person cut the scene."}
```

`under` is a stage heading's words (made when there's none) or a task's id (a subtask); an id that isn't in the plan
is refused, not made into a stage. Each answer lists the open tasks again, with the new ids.

## Big work: subtasks, checkboxes and progress

- **Subtasks** for steps that each take real work or go to different agents: `plan_add` with `under:` the parent's id.
  A parent with `[/]` (or `[%]`) in its words gets a cookie that counts its closed subtasks: `[2/5]`, `[40%]`.
- **Checkboxes** for small steps inside one task (`checks` on `plan_add`, `add_check` on `plan_update`): `- [ ]` and
  `- [X]` under the heading. With no subtasks, the cookie counts these.
- Keep it to two levels under a stage where you can. A plan the person can skim beats a complete one.

## What the person sees

The to-do list in the app (the checkbox in a feed's header, and the Inbox's for every project) is this file: one line
a task with its checkbox, its words and who has it; subtasks under their parents; done tasks folded under a count.
They add, tick, reword, reorder and remove tasks there, and those are edits to `plan.org` too, so read the plan before
you change it rather than trusting what you read earlier. The edits sent on items (comments to work) are listed with
the plan's tasks; those you mark with `comment_task` ([collaborate.md](collaborate.md)), not in the plan.

## Extending it: @raea/task

`@raea/task` (raea's `sdk/task`) is the plan in TypeScript, and what the app itself uses. A plugin or a script can:

```ts
import { readPlan, applyEdits, openTasks, definePlanKind, defineKeywords, defineProperty, checkPlan } from "@raea/task"
import { usePlan, PlanOutline, Agenda, BoardLanes } from "@raea/task/react"

defineKeywords({ name: "shots", words: "SHOT LIT GRADED | APPROVED" })
defineProperty({ key: "SHOT", about: "the shot's number", check: (v) => (/^\d+[a-z]?$/.test(v) ? undefined : "a shot number, like 12 or 12a") })
definePlanKind({ name: "shot", keywords: "shots", props: ["SHOT"], defaults: { tags: ["shot"] } })

const plan = readPlan(text)                       // every heading, every task, nothing lost
const { text: next } = applyEdits(text, [         // edits by id; every line it didn't touch is kept byte for byte
  { op: "add", title: "Shot 12: the door", kind: "shot", props: { SHOT: "12" }, under: "Look" },
  { op: "state", ref: "t-k3x9q2ab", state: "done", item: "i-door-3" },
])
checkPlan(readPlan(next))                         // what breaks the kinds' and properties' rules
```

A word of a plugin's sequence written into a plan adds that sequence's `#+TODO:` line to the file's head, so Emacs
cycles it too. `applyEdits` keeps every line it didn't change byte for byte; a heading whose keyword changes keeps its
spacing and its tags' column, as Org's own `org-todo` does.

`board`, `agenda` and `outline` are views of a plan; `defineView` adds one, and `useView(plan, name)` builds it in
React. `answerPlanTool` answers the four plan tools on any text, for a host of your own.

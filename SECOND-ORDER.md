# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

The code and tests are the work; this file is the record about the work. It is where a decision the agent made alone, an assumption that turned out wrong, or something only the person can do becomes visible to the person. With the plan approved in advance, it is the main place those decisions surface before the person uses the tool. [`docs/procedure.md`](https://github.com/sgilson7/builder-setting/blob/main/docs/procedure.md#the-second-order-notebook) says why.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). Number rows by round (R1.1, R1.2, then R2.1 after the first triage) so a later round does not renumber an earlier one. **Rule** is the rule the row produced for later sessions, and where it now lives (`CLAUDE.md`, a test, `tests/breaks.json`), or blank. **Status** is *done*, *open*, or *person*; every *person* row is named in `HANDOFF.md`. `scripts/notebook_check.py` checks the form of each row and that the notebook is not empty once `PLAN.md` holds a decision, not what the rows say.

| # | Kind | Assumed or found | What is true | What changed | Rule | Status |
| --- | --- | --- | --- | --- | --- | --- |
| R1.1 | finding | The untouched template passes `./scripts/check.sh`. | Steps 1-8 pass (38 core tests); step 9, the browser check, stops because the system `python3` has no Playwright. `./scripts/setup.sh` has not been run here. | Nothing yet. Setup runs before the build (stage 4). | | open |
| R1.2 | divergence | The description goes into `BRIEF.md` word for word. | It names a peer by first name, and Builder keeps names of people out of a repository that may be public. | The name is replaced with `[a peer]`; the rest is unchanged. | | done |
| R1.3 | finding | `builder-setting/` would stay a clone. | Its `.git` was removed as asked, so changes to Builder there cannot be diffed against upstream or sent as a pull request. | The Builder additions wait on GL-Q6. | | person |
| R1.4 | finding | The syllabus schedules a prototype review. | It does not. The announcement promises students will review the game and propose changes; the syllabus says the instructor builds after submission and the grade does not depend on it. | The mockups show a proposed review between C5 (30 Nov) and C6 (7 Dec), marked as a proposal. | | person |
| R1.5 | finding | The MerryQuery site address was given. | No address was in the request. GitHub lists `benneigh/MerryQuery` with the homepage exploremq.benyamintabarsi.com; the mockups follow that site. | GL-Q3 asks you to confirm it is the site you meant. | | person |
| R1.6 | finding | The project is worth 20% of the course grade. | The syllabus puts it in the Capstone Projects category (20%) beside three SAM capstones of 100 points each, so with an unweighted average it counts for about 5%. | The mockup says "one of the four Capstone Projects" and gives no percentage. | | person |
| R1.7 | finding | The root `CLAUDE.md` of Builder lets an agent edit the reference repository. | It says not to edit it when a person describes a tool, and to follow its own rules (sync, planted defects) when working on it. You asked for both: a tool, and changes to Builder. | The homepage is built here; the Builder additions follow the reference repository's rules (GL-Q6). | | done |
| R1.8 | finding | Student files would arrive as Builder needs them. | The syllabus asks for three files but fixes no names, headings or sheet names, so an intake script would have to guess. | The mockups propose fixed template files (GL-Q4). | | person |
| R1.9 | finding | Contribution evidence can sit in the shared folder. | A Team sheet with names sits in the same workbook the intake reads, and the games may be public. | Proposed rule GL-5: the intake drops the Team sheet and document author metadata before anything is copied into a project. | GL-5, a test in the intake kit (planned) | open |
| R1.10 | finding | The literature review was written from sources I opened. | A research subagent searched and checked each DOI against Crossref and read abstracts; I did not open every paper myself. Five entries carry caveats. | The literature tab says so and marks each caveat. | | done |
| R1.11 | divergence | Each draft comes due just after SAM teaches its app (my first mockup copy). | True for Word (draft 16 Nov, Word capstone 2 Nov) and Excel (draft 23 Nov, Excel modules by 16 Nov). The PowerPoint draft is due 30 Nov, the same day as SAM PowerPoint Modules 1-2. | Mockup copy and the timeline note now say so. | GL-1: dates and claims about them come from schedule data with a test | done |

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read.

| Key | String |
| --- | --- |

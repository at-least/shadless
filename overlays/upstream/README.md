# overlays/upstream — source-level patches on the pinned upstream

Unified diffs applied to `.upstream/shadcn-ui` **before** conversion, with
`git apply --3way`. Use this kind for content that has to differ from
upstream in the source itself (an example's copy, an mdx sentence a
transform cannot express). Prefer a `rule` in `src/` when the change is
structural.

- Create: edit the file under `.upstream/shadcn-ui`, then
  `git -C .upstream/shadcn-ui diff -- <path> > overlays/upstream/NNN-<slug>.patch`
- Audit: `node gates/overlay.mjs --audit` — each patch must reverse-apply
  cleanly (i.e. be applied) or it lands in the `conflict` bucket.
- Re-pin: `pipeline upstream` applies the series onto the new tag with
  3-way merge; conflicts come back as task packets.

The series is empty today. The mechanism exists so that the next such need
has a home that survives re-pins, instead of a find/replace on output.

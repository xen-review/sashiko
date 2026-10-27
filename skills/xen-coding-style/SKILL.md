---
name: xen-coding-style
description: "Apply authoritative Xen CODING_STYLE and reviewer-oriented spacing when reviewing Xen code or drafting small functional suggestions."
---

# Xen Coding Style Reference

Read the actual reviewed checkout's CODING_STYLE. It is authoritative; this
portable sample does not vendor or replace it. Respect imported-code and
subsystem exceptions. Style guidance is not permission to reformat a patch,
modify source, or add cosmetic findings to a functional review.

Use four-space block indentation, spaces inside control-flow parentheses,
spaces around operators and Xen's brace rules. Keep lines below 80 columns
except documented searchable-string exceptions. Distinguish continuation
alignment from block indentation; preserve tabs where an imported subsystem
requires them. Use exact types, format specifiers, macro widths and applicable
const or lifetime annotations only after checking declarations and callers.

Keep related declarations, setup and immediate error checks together. Separate
independent topics, large assertions or long calls from adjacent work with one
blank line. Do not add blank lines after opening braces, before closing braces,
between a condition and its body, or between a closing brace and else. Do not
split every assignment and check into separate topics.

Use C comments that explain the reason or non-obvious invariant rather than
narrating code. Keep implementation comments concise; preserve meaningful API
ownership, side-effect and special-value contracts. Do not duplicate lock
assertions or parameter lists as empty narration. Never add copyright or
license headers, endorsements or sign-offs to suggested code without a request.

For an incremental functional diff, retain sufficient exact context, applicable
conditional compilation and coherent nearby hunks. Review added and removed
lines against that patch's own committed tree. Do not silently substitute a
later revision or claim compilation, style checks or runtime testing that did
not happen. Builds, formatters, source edits and automatic fixes need separate
authorization; this reference alone grants none.
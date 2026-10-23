# Native Xen Review Profile

## Goal

Run an isolated daemon instance that reviews Xen submissions from xen-devel
using the authenticated Codex CLI. Xen must be an explicit project, not a Linux
instance pointed at a different repository.

## Boundaries

- Add a typed `xen` project selector shared by the daemon, worker and CLI.
- Give Xen its own bundled review guidance and review workflow. Reuse the
  existing source tools, structured finding types and verification helpers,
  without presenting Linux-specific rules as Xen requirements.
- Inspect guest/hypervisor trust boundaries, guest-controlled lengths and
  addresses, privilege checks, domain/vCPU lifetime, locking, memory ordering,
  architecture differences and toolstack error handling.
- Report only concrete defects supported by repository evidence. Preserve
  dismissed concerns and require verification before publishing findings.
- Select baselines from explicit base commits, configured Xen remotes and the
  repository's default branch, retaining the local checkout as a final fallback
  when remote HEAD is unavailable. Never add linux-next or Linus's tree for Xen.
- Keep existing Linux and Sashiko project behavior unchanged.
- Disable Linux bug discovery and Linux-specific MAINTAINERS authority for Xen.
- Track only xen-devel; default outbound delivery to muted, with no SMTP
  configured. Bind the installed instance to loopback.

## Deployment

Store account configuration under ~/.config/sashiko and runtime state under
~/.local/share/sashiko. Use a dedicated Xen clone and worktree directory, a
separate database stamped with the Xen project identity, and a user systemd
service. Use one concurrent review initially to limit subscription usage.
Do not backfill the full mailing-list archive on first startup.

Use https://github.com/xen-review/xen as the clone's origin so it remains the
future push destination. Add the authoritative Xen repository as upstream for
read-only baseline refreshes. Do not push or configure credentials during setup.

## Verification

Add focused tests for project parsing, workflow selection, prompt inclusion,
finding verification and Xen baseline isolation. Run the repository's PR
checks and local Sashiko self-review before deployment. Verify service startup,
loopback HTTP access, xen-devel ingestion and an actual Codex-backed review.
Do not enable public posting or run benchmark suites as part of installation.

## Design Review

A separate profile avoids misleading Linux findings and database identity
mixups. A separate workflow avoids changes to Linux review-quality heuristics.
Shared typed finding and Git tool APIs keep the implementation bounded. The
main integration risks are subprocess project propagation, hardcoded Linux
baseline fallbacks, and progress/report consumers assuming two projects; the
focused tests and feature-profile checks must cover those boundaries.
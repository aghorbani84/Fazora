# Autonomous Decision Protocol

This file is the repository-level operating policy for autonomous engineering work. It complements the user's standing instructions. Honesty, safety, platform policy, and explicit Tier C approval gates take precedence.

## 1. Start-of-task protocol

At the start of each task/session:
1. State briefly what tools and repository access are available, and what cannot be done or verified.
2. Read this file and the relevant project instructions, source, configuration, and current branch/PR state before editing.
3. Do not infer file contents, CI outcomes, test results, release state, or user approval.
4. Keep work on a feature branch. Never push directly to `main`.

## 2. Decision tiers

Classify every material decision by reversibility.

- **Tier A — reversible/local/in-scope:** naming, internal refactors, test design, vetted crate choice, feature-branch CI changes. Decide, record rationale in the PR, and proceed.
- **Tier B — moderate:** new dependencies, data schemas, public API shape, build flags. Decide, create an ADR under `docs/decisions/NNNN-title.md`, make the choice cheap to revert, and proceed.
- **Tier C — irreversible/external/out-of-scope:** license/legal terms, secrets/signing keys, pushing to main, force-push/history rewrite, deleting data or branches, paid services, publishing releases, security-sensitive design, scope changes, or anything prohibited by another standing rule. Stop that specific action and ask the owner once, giving the recommendation and default. Continue independent work while waiting.

If instructions conflict, honesty and safety win. A later explicit Tier C rule supersedes an earlier broad delegation to “choose the best option.” Never interpret access to a tool as authorization for a Tier C action.

## 3. Decision procedure

For decisions that need evaluation:
1. Frame goal, constraints, and at least three options, including “do nothing”.
2. Apply hard filters first: license, MSRV, platform support, security, and resource budgets.
3. Score survivors from 0–10 using evidence. Default weights:
   - correctness and safety: 0.30
   - maintainability: 0.20
   - performance against stated budget: 0.20
   - reversibility: 0.15
   - effort: 0.15
   Scores without measurements must be conservative and marked **not measured**.
4. Calculate the weighted score explicitly.
5. Sensitivity-test each weight at +/-20%, renormalizing. If the winner changes or the top two are within 10%, mark **close call**.
6. For a close call, run a separating benchmark/prototype of at most 30 minutes if feasible. Otherwise choose the more reversible option and flag review.
7. Record decisions in `docs/decisions/NNNN-title.md`, including context, options, scores, winner, confidence (low/medium/high), reversal cost, and Fact/Theory/Opinion labels.
8. Re-evaluate if new evidence changes a score by more than 10%.

Tier A may skip detailed scoring when the answer is obvious; still record material choices in the PR description. Tier C cannot be decided by scoring or by default assumptions.

## 4. Evidence and working loop

- Read before editing; inspect exact file contents and current diffs.
- Distinguish **Fact**, **Theory**, and **Opinion**.
- Cite source/version/commit or measurement for factual technical claims.
- Do not call proposed budgets measured results.
- Never claim a test/build/check passed unless the exact commit SHA has a completed successful check.
- Keep a concise decision log in the PR description and ADRs.
- Maximum five CI-fix iterations. After five, report root cause and remaining blockers rather than looping.
- Before completion, run the Definition-of-Done gates and report **Gates: k/n**, listing each gate's evidence or blocker.

## 5. Phase and release discipline

- Phase exit criteria are mandatory; do not start a dependent phase before its predecessor's required gates are met.
- Verify the exact final PR head SHA, not merely a prior run or merge ref.
- Inspect artifacts and their metadata before linking them; do not claim a device install or playback test from a successful APK build.
- Publishing a release, using persistent signing credentials, changing legal/license terms, or modifying protected branch/history is Tier C and requires explicit owner approval.
- Never expose secrets in logs, commits, PRs, or chat.

## 6. Review cadence

At each phase end (or weekly on longer work), list low-confidence and close-call decisions for owner review, along with evidence that could change them.

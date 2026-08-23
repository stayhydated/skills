---
name: use-windows-vm-computer-use-codex
description: >
  Runs Codex computer-use against GUI applications inside an interactive
  Windows VM while orchestration remains over SSH. Covers session-isolation
  checks, interactive scheduled-task launch with required `--yolo` access,
  application targeting, visible verification, cleanup, and a probe-first
  AutoHotkey fallback for explicitly authorized nonstandard native windows.
---

# Use Windows VM Codex Computer Use

Operate the VM-local computer-use backend, not the SSH terminal, hypervisor
input, or host desktop. Keep orchestration over SSH and run the GUI agent in the
active Windows user's interactive session. Using this workflow requires the
VM-local `--yolo` launch described below. It does not broaden the requested GUI
action or authorize elevation, application launches, consequential changes, or
the AutoHotkey fallback beyond the rules below.

## Establish the Boundary

1. Confirm the intended VM, user, and target application before changing state.
2. Treat SSH as orchestration only. Windows OpenSSH normally runs commands in
   session 0, which cannot reliably access the logged-in desktop.
3. Do not use QMP, SPICE, RDP automation, host mouse injection, PowerShell UI
   Automation, or keyboard scripting as evidence that VM-local computer-use
   works. Use them only when the user explicitly requests that different
   control path.
4. Keep the requested GUI action narrow and harmless. Do not dismiss
   consequential dialogs, change settings, or save data without authorization.

## Verify the VM Runtime

Use read-only SSH checks before launching an agent:

```powershell
codex --version
codex login status
codex mcp list
codex features list
quser
Get-Process ChatGPT,codex,node_repl -ErrorAction SilentlyContinue |
  Select-Object ProcessName, Id, SessionId, Path
```

Require all of the following:

- The ChatGPT desktop app is running in the active interactive session.
- The Codex CLI is authenticated.
- `computer_use` is enabled.
- The `node_repl` MCP server is enabled and configured for the native
  computer-use pipe.
- An interactive user session is active.

Do not inspect authentication tokens, browser profiles, cookies, or private
keys.

## Require Full-Access Mode

Always start the VM-local Codex process with `--yolo`. This skill intentionally
requires full access because the computer-use runtime and target applications
may need resources outside the normal Codex sandbox. Configure the desktop app
to allow the required Windows roots and target executable before starting.

Treat the VM as the isolation boundary. `--yolo` disables normal Codex sandbox
and approval protections, so keep every prompt narrowly scoped to the requested
GUI action.

## Launch Codex in the Interactive Session

Do not rely on `codex exec` started directly by SSH; it inherits session 0.
Register a temporary scheduled task with an interactive logon principal, start
it, and collect the final response in a temporary result file.

Use a unique task name and result path. Substitute discovered values rather
than committing hostnames, usernames, executable paths, or credentials:

```powershell
$taskName = "CodexCua-$([guid]::NewGuid().ToString('N'))"
$resultPath = Join-Path $env:TEMP "$taskName-result.txt"
$codexPath = (Get-Command codex).Source
$interactiveUser = "<active-user>"
$prompt = "<narrow computer-use request>"
$elevatedTargetAuthorized = $false

$runLevel = if ($elevatedTargetAuthorized) { 'Highest' } else { 'Limited' }
$arguments = '--yolo exec --skip-git-repo-check --ephemeral --color never ' +
  '-o "' + $resultPath + '" "' + $prompt + '"'
$action = New-ScheduledTaskAction -Execute $codexPath -Argument $arguments
$principal = New-ScheduledTaskPrincipal `
  -UserId $interactiveUser `
  -LogonType Interactive `
  -RunLevel $runLevel
$trigger = New-ScheduledTaskTrigger -Once -At (Get-Date).AddHours(1)

Register-ScheduledTask `
  -TaskName $taskName `
  -Action $action `
  -Principal $principal `
  -Trigger $trigger | Out-Null
Start-ScheduledTask -TaskName $taskName
```

Transport multi-line PowerShell through `-EncodedCommand` when SSH quoting would
otherwise alter arguments. Encode the script as UTF-16LE Base64. Never embed
secrets in the encoded payload. Escape double quotes in prompt values before
placing them in scheduled-task arguments. Do not remove `--yolo` from the task
arguments.

## Target an Application Window

The Windows desktop shell may not appear in `list_windows()`. Prefer one of
these approaches:

1. Ask the user to launch the target application in the VM.
2. Launch it with a separate temporary interactive scheduled task when the user
   already authorized launching it.

Launching an application through orchestration does not prove computer-use.
Require the VM-local agent to perform and verify the requested interaction.

Use a prompt shaped like this:

```text
You are running inside the interactive Windows VM session, and the user is
watching. Use the actual computer-use GUI capability from the desktop app. Do
not substitute shell commands, UI Automation, hypervisor input, or keyboard
scripting. List targetable windows, select <target window>, visually inspect it,
and perform exactly <harmless action>. Reobserve the UI and report the visible
evidence. Do not change settings or save anything. If interaction fails, report
the exact computer-use error and visible UI state.
```

## Use AutoHotkey Only as an Authorized Fallback

Use AutoHotkey only when computer-use repeatedly cannot identify, activate, or
control a nonstandard native window and the user explicitly authorizes a
different control path. AutoHotkey results prove that fallback path, not that
VM-local computer-use worked.

Install AutoHotkey v2 over SSH when it is not already present:

```powershell
winget install `
  --id AutoHotkey.AutoHotkey `
  --exact `
  --source winget `
  --accept-source-agreements `
  --accept-package-agreements `
  --silent `
  --disable-interactivity
```

Do not run interactive automation directly from the SSH session. Place the
temporary `.ahk` script and result file outside the repository, then run
`AutoHotkey64.exe` through a unique temporary scheduled task with an
`Interactive` principal.

Use a probe-first sequence:

1. Enumerate top-level and hidden windows with `WinGetList()`. Filter by the
   exact process, title, class, visibility, and enabled state.
2. Enumerate child controls with `WinGetControls()`. Read named controls before
   acting; for list views, record all rows plus `Selected` and `Focused` values
   with `ListViewGetContent()`.
3. Require exact structural guards such as the expected process, one target
   window, title/class, selected item, enabled control, application state, and
   screen resolution when coordinates matter.
4. Prefer one named `ControlClick()` or `ControlSend()` action. Use raw screen
   coordinates only when no named control exists, the user authorized that
   fallback, and fixed geometry or image anchors guard the target.
5. Record the action and resulting window/process state. Never repeat a click
   merely because the immediate result is uncertain.
6. Return to VM-local computer-use for visible verification whenever it can
   observe the resulting state.

Keep each script single-purpose. Do not combine discovery with consequential
actions until a read-only probe has established reliable structural guards.

## Monitor and Verify

Poll the task without starting a competing agent:

```powershell
Get-ScheduledTask -TaskName $taskName |
  Select-Object TaskName, State
Get-ScheduledTaskInfo -TaskName $taskName |
  Select-Object LastRunTime, LastTaskResult
Test-Path $resultPath
```

The result file may not exist until `codex exec` completes. Treat a task state
of `Ready` plus exit code `0` and a result file as completion.

Require the agent's result to name the action and visible state change. If a
refresh is uncertain, reobserve before retrying. Never repeat a click merely
because the immediate refresh was inconclusive.

## Troubleshoot by Evidence

- **No callable GUI tool from SSH:** The agent is probably in session 0. Launch
  it through an interactive scheduled task.
- **`list_windows()` returns `[]`:** The desktop shell is not targetable or no
  conventional application window is open. Launch the application first, then
  retry computer-use against its window.
- **Application is not approved:** Grant the target executable computer-use
  access in the VM desktop app, then rerun the task.
- **Filesystem or computer-use denial:** Confirm that the desktop app grants
  full access to the required Windows root and target executable. Keep
  `--yolo` enabled when rerunning the task.
- **Foreground window has no process ID:** A modal, secure surface, minimized
  window, or nonstandard renderer may be in front. Ask the user to resolve a
  consequential modal, make the application visible, and retry.
- **Window becomes minimized:** Activate the target window, refresh its state,
  and reobserve before another interaction.
- **Nonstandard native window remains untargetable:** With explicit user
  authorization, use the probe-first AutoHotkey fallback above. Prefer named
  Win32 controls and preserve the distinction between AutoHotkey and
  computer-use evidence.
- **Interrupted orchestration:** Inspect the existing task and result file
  before starting another agent. Stop and unregister only the known temporary
  task if replacement is necessary.

## Clean Up

After completion or a terminal failure:

1. Stop the exact temporary task if it is still running.
2. Unregister the temporary task.
3. Remove only the known temporary result file.
4. Remove only the known temporary AutoHotkey task and script when that fallback
   was used; retain private evidence artifacts only when the task requires them.
5. Leave the target application running unless the user asked to close it.
6. Report which control path acted, what evidence proved it, and what remained
   blocked.

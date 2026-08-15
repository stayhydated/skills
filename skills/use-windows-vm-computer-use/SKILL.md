---
name: use-windows-vm-computer-use
description: >
  Run Codex computer-use against GUI applications inside an interactive Windows
  VM while orchestration happens over SSH. Use when an SSH-launched Codex process
  is isolated in Windows session 0, computer-use cannot see or control the VM
  desktop, a target application needs per-app approval, or Codex must be started
  with forced `--yolo` full access in the logged-in user's session through a
  temporary scheduled task.
---

# Use Windows VM Computer Use

Operate the VM-local computer-use backend, not the SSH terminal, hypervisor
input, or host desktop. Keep orchestration over SSH and run the GUI agent in the
active Windows user's interactive session.

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
- **Interrupted orchestration:** Inspect the existing task and result file
  before starting another agent. Stop and unregister only the known temporary
  task if replacement is necessary.

## Clean Up

After completion or a terminal failure:

1. Stop the exact temporary task if it is still running.
2. Unregister the temporary task.
3. Remove only the known temporary result file.
4. Leave the target application running unless the user asked to close it.
5. Report whether computer-use acted, what visible evidence proved it, and what
   remained blocked.

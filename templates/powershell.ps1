# START generated Lacy shell config
function {{ lacy_cmd }} {
    param($Query = "")
    $newPath = lacy prompt {{ return_all }}-- "$Query"
    if ($newPath -eq "~") {
        {{ cd }} ~
    }
    {% if custom_fuzzy.enabled %}
    elseif ($newPath.Contains("`n")) {
        $selected = $newPath | {{ custom_fuzzy.cmd }}
        if ($selected) {
            {{ cd }} "$selected"
        }
    }
    {% endif %}
    elseif (Test-Path -Path $newPath -PathType Container) {
        {{ cd }} "$newPath"
    }
    else {
        Write-Error "Error: No matching directory found for '$Query'"
    }
}

function __{{ lacy_cmd }}_autocomplete {
    param($wordToComplete, $commandAst, $cursorPosition)
    $dirs = lacy complete --basename -- "$wordToComplete"
    $dirs -split "`n" | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}

Register-ArgumentCompleter -CommandName '{{ lacy_cmd }}' -ParameterName 'Query' -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $cursorPosition)
    __{{ lacy_cmd }}_autocomplete $wordToComplete $commandAst $cursorPosition
}

Register-ArgumentCompleter -CommandName 'lacy' -ParameterName 'command' -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $cursorPosition)
    $commands = @('prompt', 'complete', 'init', 'help')
    $commands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
}
# END generated Lacy shell config

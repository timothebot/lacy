# START generated Lacy shell config
function {{lacy_cmd}} {
    param(
        [Parameter(ValueFromRemainingArguments=$true)]
        $QueryParts
    )
    
    # Join parts and normalize slashes
    $RawQuery = "$QueryParts" -replace '/', '\'
    
    # If the query starts with a slash, check if it's a real root folder.
    # If C:\[string] doesn't exist, remove the slash so Lacy searches locally.
    if ($RawQuery.StartsWith('\')) {
        if (-not (Test-Path -Path $RawQuery -PathType Container)) {
            $FullQuery = $RawQuery.TrimStart('\')
        } else {
            $FullQuery = $RawQuery
        }
    } 
    # Handle relative paths
    elseif ($RawQuery.StartsWith('.\')) {
        $FullQuery = Resolve-Path -Path $RawQuery -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Path
        if (-not $FullQuery) { $FullQuery = $RawQuery }
    }
    else {
        $FullQuery = $RawQuery
    }
    
    try {
        # Get output from lacy
        $rawOutput = lacy prompt {{return_all}}-- "$FullQuery"
        
        if ([string]::IsNullOrWhiteSpace($rawOutput)) {
            if (Test-Path -Path $RawQuery -PathType Container) {
                {{cd}} "$RawQuery"
            }
            return 
        }

        $newPath = @([string]$rawOutput)[-1].Trim()
    }
    catch {
        return
    }

    # Handle Home shortcut
    if ($newPath -eq "~") {
        {{cd}} ~
        return
    }

    {% if custom_fuzzy.enabled %}
    # Handle Fuzzy Finder
    if ($rawOutput -match "\n") {
        $selected = $rawOutput | {{custom_fuzzy.cmd}}
        if ($selected) { {{cd}} "$selected" }
        return
    }
    {% endif %}

    if (Test-Path -Path $newPath -PathType Container) {
        {{cd}} "$newPath"
    }
}
# END generated Lacy shell config
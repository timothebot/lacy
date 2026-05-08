# START generated Lacy shell config
module lacy {
    def lacy-query-to-completion [context: string] {
        let query = ($context | split row " " | skip 1 | str join " ")
        if $query == "" {
            return (ls -a  | where type == dir | get name)
        }
        if $context ends-with " " {
            return (lacy complete --basename -- $"($query) " | split row " " | uniq)
        }
        return (lacy complete --basename -- $query | split row " " | uniq)
    }

    def lacy-dir-names [context: string] {
        {
            options: {
                case_sensitive: false,
                completion_algorithm: fuzzy,
                sort: false,
            },
            completions: (lacy-query-to-completion $context)
        }
    }

    export def --env {{ lacy_cmd }} [...args: string@lacy-dir-names] {
        let query = ($args | str join " ")
        let new_path = (lacy prompt {{ return_all }}-- $"($query)")
        if $new_path == "~" {
            {{ cd }} ~
        {% if custom_fuzzy.enabled %} } else if ($new_path | str contains "\n") {
            let selected = ($new_path | {{ custom_fuzzy.cmd }})
            if ($selected | path exists) and ($selected | path type) == "dir" {
                {{ cd }} $selected
            }
            # Even though path type normally also checks for path existing,
            # for some reason (nothing) | path type is "dir". Likely a bug.
        {% endif %} } else if ($new_path | path exists) and ($new_path | path type) in ["dir", "symlink"] {
            {{ cd }} $new_path
        } else {
            print $"Error: No matching directory found for '($query)'"
        }
    }
}

use lacy {{ lacy_cmd }}
# END generated Lacy shell config

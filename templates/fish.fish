# START generated Lacy shell config
function {{ lacy_cmd }}
    set new_path (lacy prompt {{ return_all }}-- "$argv")
    if test "$new_path" = "~"
        {{ cd }} ~
    {% if custom_fuzzy.enabled %}else if string match -q "*\n*" -- $new_path
        set selected (printf "%s\n" $new_path | {{ custom_fuzzy.cmd }})
        if test -n "$selected"
            {{ cd }} "$selected"
        end
    {% endif %}else if test -d "$new_path"
        {{ cd }} "$new_path"
    else
        echo "Error: No matching directory found for '$argv'"
    end
end
function __{{ lacy_cmd }}_autocomplete
    set args $argv
    if test "$args" = ""
        ls -D --icons=never -1
    else
        set dirs (string split ' ' (lacy complete -- "$args"))
        for dir in $dirs
            basename $dir
        end
    end
end
complete --no-files lacy -x -a "prompt complete init help"
complete --no-files {{ lacy_cmd }} -r -a "(__{{ lacy_cmd }}_autocomplete)"
# END generated Lacy shell config
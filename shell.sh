function y {
    new_path=$(lacy $*)
    if [ -d "$new_path" ]; then
        echo $new_path
        cd "$new_path"
    else
        echo "Error: No matching directory found for '$*'"
    fi
}

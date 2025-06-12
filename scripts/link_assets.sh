#!/bin/sh

repo_root=$(git rev-parse --show-toplevel 2> /dev/null)

if [ -z "$repo_root" ]; then
    echo "No git repository found."
    exit 1
fi

create_symlinks() {
    target_dir=$1
    pattern="${2-*}"
    negative_pattern="${3}"

    if [ -d "$repo_root/$target_dir" ]; then
        for dir in "$repo_root"/"$target_dir"/$pattern/; do

            if [ -n "$negative_pattern" ]; then
                dir_name=$(basename "$dir")
                # shellcheck disable=SC2254
                case "$dir_name" in
                    $negative_pattern) 
                        continue
                        ;;
                esac
            fi

            if [ -d "${dir}assets" ]; then
                echo "An assets directory already exists at ${dir}. Skipping..."
            else                
                relative_path=$(realpath --relative-to="$dir" "$repo_root/assets")
                ln -s "$relative_path" "${dir}assets"
                echo "Created symlink for ${dir}assets"
            fi
        done
    else
        echo "Directory $target_dir not found."
    fi
}

create_symlinks "examples"
create_symlinks "games"
create_symlinks "crates" "*" "rx_bevy*"

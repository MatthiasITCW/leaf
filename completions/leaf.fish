complete -c leaf -f

complete -c leaf -s h -l help -d "Show help message and exit"
complete -c leaf -s V -l version -d "Show version information and exit"
complete -c leaf -s w -l watch -d "Watch file for changes and reload"
complete -c leaf -l theme -x -a "arctic forest ocean-dark solarized-dark" -d "Set color theme preset"
complete -c leaf -s e -l editor -x -a "nano vim vi nvim micro helix emacs jed code codium subl gedit kate mousepad zed xjed notepad notepad++" -d "Set external editor"
complete -c leaf -l inline -x -a "ansi plain" -d "Render to stdout (no TUI)"
complete -c leaf -l picker -d "Open the file browser picker"
complete -c leaf -l config -d "Open configuration file in editor"
complete -c leaf -l update -d "Update leaf to the latest version"
complete -c leaf -l auto-complete -d "Install shell completions for leaf"

complete -c leaf -F -a "(__fish_complete_suffix .md)"

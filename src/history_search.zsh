# Will be run before every prompt draw
hsearch_precmd() {
    echo -n \
        $$ \
        "@@@" \
        "$(date "+%s")" \
        "@@@" \
        "\"$(pwd -P)\"" \
        "@@@ " >> $ETERNAL_HISTORY_FILE
    # must do it in seperate line otherwise newline is not properly escaped
    history | tail -1 | perl -pe 's/^\s*\d+\s*//' >> $ETERNAL_HISTORY_FILE

}
# If precmd/preexec arrays are not already set, set them.
[[ -z "${precmd_functions+1}" ]] && precmd_functions=()

# If hsearch precmd/preexec functions are already hooked, don't double-hook them
# to avoid unnecessary performance degradation in nested shells
if [[ -z ${precmd_functions[(re)hsearch_precmd]} ]]; then
    precmd_functions+=(hsearch_precmd)
fi

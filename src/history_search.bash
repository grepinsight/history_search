export ETERNAL_HISTORY_FILE=$HOME/.bash_eternal_history
record_external_history() {
    echo -n \
        $$ \
        "@@@" \
        "$(date "+%s")" \
        "@@@" \
        "\"$(pwd -P)\"" \
        "@@@ " >> $ETERNAL_HISTORY_FILE
    # must do it in seperate line otherwise newline is not properly escaped
    history 1 | perl -pe 's/^\s*\d+\s*\d+\s*//' >> $ETERNAL_HISTORY_FILE

    # call out to the previous PROMPT_COMMAND, if there was one.
    "$@"
}

if [[ ! "${PROMPT_COMMAND:-}" =~ "^record_external_history" ]]; then
    export PROMPT_COMMAND="record_external_history ${PROMPT_COMMAND:+$PROMPT_COMMAND}"
fi

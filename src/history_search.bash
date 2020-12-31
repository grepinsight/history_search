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
}

PROMPT_COMMAND="${PROMPT_COMMAND:+$PROMPT_COMMAND ; }"record_external_history

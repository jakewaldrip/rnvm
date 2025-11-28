rnvm() {
  if [ $# -eq 0 ]; then
    command rnvm --help
    return 1
  fi

  local sub_command="$1"
  shift

  if [ "$sub_command" = "use" ] || [ "$sub_command" = "start" ]; then
    local output
    output=$(command rnvm "$sub_command" "$@" 2>&1)
    cmd_status=$?

    if [ $cmd_status -ne 0 ]; then
      echo "$output" >&2
      return $cmd_status
    fi

    eval "$output"
  else
    command rnvm "$sub_command" "$@"
  fi
}

# Run on startup to source path
start_output=$(command rnvm start)
cmd_status=$?
if [ $cmd_status -ne 0 ]; then
  echo "$start_output" >&2
else
  eval "$start_output"
fi

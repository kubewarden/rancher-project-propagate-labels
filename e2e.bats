#!/usr/bin/env bats

@test "Mutate Namespace because Project has more labels" {
  run kwctl run \
    -r test_data/ns_without_labels.json \
    --allow-context-aware \
    --replay-host-capabilities-interactions test_data/session_project_found.yml \
    annotated-policy.wasm
  [ "$status" -eq 0 ]
  echo "$output"
  [ $(expr "$output" : '.*"allowed":true.*') -ne 0 ]
  [ $(expr "$output" : '.*JSONPatch.*') -ne 0 ]
}

@test "Does not mutate Namespace because Project has already all the labels" {
  run kwctl run \
    -r test_data/ns_with_labels.json \
    --allow-context-aware \
    --replay-host-capabilities-interactions test_data/session_project_found.yml \
    annotated-policy.wasm
  [ "$status" -eq 0 ]
  echo "$output"
  [ $(expr "$output" : '.*"allowed":true.*') -ne 0 ]
  [ $(expr "$output" : '.*JSONPatch.*') -eq 0 ]
}

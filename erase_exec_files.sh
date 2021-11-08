#!/bin/bash
find . -type f | grep -E "^\./[^\./]+$" |xargs rm
find . -type f | grep -E "^\./codeforces/[^\./]+$" | xargs rm
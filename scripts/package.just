#!/usr/bin/env sh

package target:
    @echo "Packaging {{target}} "
    @just "package-{{target}}"

@package-web:

@package-server:


@package-android:
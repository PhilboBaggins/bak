
## bak

    #!/bin/sh

    if [ -e "$1" ]; then
        #cp --verbose --interactive --recursive --preserve=all "$1" "${1}.bak"
        cp -virp "$1" "${1}.bak"
    else
        echo "Usage: bak file" >&2
        exit 1
    fi

## bakt

    #!/bin/sh

    if [ -e "$1" ]; then
        #cp --verbose --interactive --recursive --preserve=all "$1" "${1}_$(date +%Y-%m-%d_%H%M).bak"
        cp -virp "$1" "${1}_$(date +%Y-%m-%d_%H%M).bak"
    else
        echo "Usage: bak file" >&2
        exit 1
    fi

## baktm

    #!/bin/sh

    if [ -e "$1" ]; then
        modificationDate=$(stat -c %y "$1" | cut -d. -f1 | sed 's/://g' | sed 's/ /_/g')
        #cp --verbose --interactive --recursive --preserve=all "$1" "${1}_${modificationDate}.bak"
        cp -virp "$1" "${1}_${modificationDate}.bak"
    else
        echo "Usage: bak file" >&2
        exit 1
    fi

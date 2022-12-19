#!/bin/bash
slice_script=$(dirname "$0")/slice_tilesheet.sh
${slice_script} content/base/tilesets/mystic_mountain mystic_mountains_tiles.png tiles/mystic_mountain_ 64

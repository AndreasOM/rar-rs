#!/bin/bash

base=content/base/tilesets/grassland
tileset=grassland_tiles.png
prefix=tiles/grassland_
suffix=.png
tile_size=64

function cut_tile() {
	x=$1
	y=$2
	dst=$3
	dst=${base}/${prefix}${dst}${suffix}

	src=${base}/${tileset}
	ts=$tile_size
	sx=$((x*ts))
	sy=$((y*ts))

	echo "${ts}x${ts} @ ${sx}, ${sy} ${src} => ${dst}"
#	file ${src}
	gm convert ${src} -crop ${ts}x${ts}+${sx}+${sy} +repage ${dst}
}


cut_tile 1 1 a_0

cut_tile 3 1 b_0
cut_tile 4 1 b_1
cut_tile 5 1 b_2

cut_tile 7 1 c_00
cut_tile 8 1 c_10
cut_tile 9 1 c_20
cut_tile 7 2 c_01
cut_tile 8 2 c_11
cut_tile 9 2 c_21
cut_tile 7 3 c_02
cut_tile 8 3 c_12
cut_tile 9 3 c_22

cut_tile 1 5 d_0

cut_tile 4 5 e_0
cut_tile 4 6 e_1
cut_tile 4 7 e_2

cut_tile 7 5 f_00
cut_tile 8 5 f_10
cut_tile 9 5 f_20
cut_tile 7 6 f_01
cut_tile 8 6 f_11
cut_tile 9 6 f_21
cut_tile 7 7 f_02
cut_tile 8 7 f_12
cut_tile 9 7 f_22
